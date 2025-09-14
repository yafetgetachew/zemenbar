use ethiopic_calendar::{EthiopianYear, GregorianYear};
use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};
use tauri_plugin_autostart::ManagerExt;
use std::time::Duration;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthiopianDate {
    pub year: usize,
    pub month: usize,
    pub day: usize,
    pub day_geez: String,
}

impl EthiopianDate {
    pub fn today() -> Self {
        let today = Local::now().date_naive();
        let gregorian = GregorianYear::new(today.year() as usize, today.month() as usize, today.day() as usize);
        let ethiopian: EthiopianYear = gregorian.into();

        let day = ethiopian.day();
        Self {
            year: ethiopian.year(),
            month: ethiopian.month(),
            day,
            day_geez: Self::to_geez_number(day),
        }
    }

    pub fn from_gregorian(year: i32, month: u32, day: u32) -> Option<Self> {
        let gregorian = GregorianYear::new(year as usize, month as usize, day as usize);
        let ethiopian: EthiopianYear = gregorian.into();

        let day = ethiopian.day();
        Some(Self {
            year: ethiopian.year(),
            month: ethiopian.month(),
            day,
            day_geez: Self::to_geez_number(day),
        })
    }

    pub fn amharic_month(&self) -> &'static str {
        match self.month {
            1 => "መስከረም",
            2 => "ጥቅምት",
            3 => "ኅዳር",
            4 => "ታኅሣሥ",
            5 => "ጥር",
            6 => "የካቲት",
            7 => "መጋቢት",
            8 => "ሚያዝያ",
            9 => "ግንቦት",
            10 => "ሰኔ",
            11 => "ሐምሌ",
            12 => "ነሐሴ",
            13 => "ጳጉሜ",
            _ => "Unknown",
        }
    }

    pub fn english_month(&self) -> &'static str {
        match self.month {
            1 => "Meskerem",
            2 => "Tikimt",
            3 => "Hidar",
            4 => "Tahsas",
            5 => "Tir",
            6 => "Yekatit",
            7 => "Megabit",
            8 => "Miazia",
            9 => "Ginbot",
            10 => "Sene",
            11 => "Hamle",
            12 => "Nehase",
            13 => "Pagume",
            _ => "Unknown",
        }
    }


    pub fn to_geez_number(num: usize) -> String {
        if num == 0 {
            return "".to_string();
        }

        let geez_digits = ["", "፩", "፪", "፫", "፬", "፭", "፮", "፯", "፰", "፱"];
        let geez_tens = ["", "፲", "፳", "፴", "፵", "፶", "፷", "፰", "፱"];

        if num < 10 {
            geez_digits[num].to_string()
        } else if num < 100 {
            let tens = num / 10;
            let ones = num % 10;
            if tens == 1 {
                if ones == 0 {
                    "፲".to_string()
                } else {
                    format!("፲{}", geez_digits[ones])
                }
            } else {
                if ones == 0 {
                    geez_tens[tens].to_string()
                } else {
                    format!("{}{}", geez_tens[tens], geez_digits[ones])
                }
            }
        } else if num < 1000 {
            let hundreds = num / 100;
            let remainder = num % 100;
            let hundred_part = if hundreds == 1 {
                "፻".to_string()
            } else {
                format!("{}{}", geez_digits[hundreds], "፻")
            };

            if remainder == 0 {
                hundred_part
            } else {
                format!("{}{}", hundred_part, Self::to_geez_number(remainder))
            }
        } else if num < 10000 {
            let thousands = num / 100; // Work with hundreds instead
            let remainder = num % 100;

            // Convert the hundreds part (10-99 hundreds)
            let hundred_part = if thousands < 10 {
                format!("{}፻", geez_digits[thousands])
            } else if thousands < 100 {
                let tens = thousands / 10;
                let ones = thousands % 10;
                if tens == 1 {
                    if ones == 0 {
                        "፲፻".to_string()
                    } else {
                        format!("፲{}፻", geez_digits[ones])
                    }
                } else {
                    if ones == 0 {
                        format!("{}፻", geez_tens[tens])
                    } else {
                        format!("{}{}፻", geez_tens[tens], geez_digits[ones])
                    }
                }
            } else {
                format!("{}፻", Self::to_geez_number(thousands))
            };

            if remainder == 0 {
                hundred_part
            } else {
                format!("{}{}", hundred_part, Self::to_geez_number(remainder))
            }
        } else {
            // For very large numbers, fall back to regular numbers
            num.to_string()
        }
    }

    pub fn day_geez(&self) -> String {
        Self::to_geez_number(self.day)
    }

    pub fn year_geez(&self) -> String {
        Self::to_geez_number(self.year)
    }
}





#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub use_amharic: bool,
    pub use_geez_numbers: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            use_amharic: true,
            use_geez_numbers: false,
        }
    }
}


#[tauri::command]
fn set_tray_text(app: tauri::AppHandle, text: String) -> Result<(), String> {
    println!("Setting tray text to: {}", text);
    if let Some(tray) = app.tray_by_id("main") {
        let _ = tray.set_title(Some(&text));
        println!("Tray text set successfully");
    } else {
        println!("Tray not found!");
    }
    Ok(())
}


fn get_settings_path(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    app.path()
        .app_data_dir()
        .map(|dir| dir.join("settings.json"))
        .map_err(|e| format!("Failed to get app data directory: {}", e))
}

#[tauri::command]
fn load_settings(app: tauri::AppHandle) -> Result<AppSettings, String> {
    let settings_path = get_settings_path(&app)?;

    if settings_path.exists() {
        let content = std::fs::read_to_string(&settings_path)
            .map_err(|e| format!("Failed to read settings file: {}", e))?;

        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse settings: {}", e))
    } else {
        Ok(AppSettings::default())
    }
}

#[tauri::command]
fn save_settings(app: tauri::AppHandle, settings: AppSettings) -> Result<(), String> {
    let settings_path = get_settings_path(&app)?;

    // Create parent directory if it doesn't exist
    if let Some(parent) = settings_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create settings directory: {}", e))?;
    }

    let content = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    std::fs::write(&settings_path, content)
        .map_err(|e| format!("Failed to write settings file: {}", e))?;

    println!("Settings saved: {:?}", settings);
    Ok(())
}

// Estimate text width (simple heuristic)
fn estimate_text_width(text: &str) -> f64 {
    let avg_char_width = 7.0; // pixels per character (rough)
    text.chars().count() as f64 * avg_char_width
}

// Update tray with current Ethiopian date using settings and compacting if needed
fn update_tray_with_date(app: &tauri::AppHandle) -> Result<(), String> {
    let settings = load_settings(app.clone())?;
    let date = EthiopianDate::today();

    let month_name = if settings.use_amharic { date.amharic_month() } else { date.english_month() };
    let day_text = if settings.use_geez_numbers { &date.day_geez } else { &date.day.to_string() };
    let year_text = if settings.use_geez_numbers { EthiopianDate::to_geez_number(date.year) } else { date.year.to_string() };

    let full_text = format!("{} {} {}", month_name, day_text, year_text);
    let month_abbrev = if settings.use_amharic {
        month_name.chars().take(2).collect::<String>()
    } else {
        month_name.chars().take(3).collect::<String>()
    };
    let compact_text = format!("{} {}", month_abbrev, day_text);

    let threshold_width = 160.0;
    let text_to_show = if estimate_text_width(&full_text) <= threshold_width { full_text } else { compact_text };

    set_tray_text(app.clone(), text_to_show)
}

#[tauri::command]
fn update_tray_display(app: tauri::AppHandle) -> Result<(), String> {
    update_tray_with_date(&app)
}




#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(window) = app.get_webview_window("settings") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, None))
        .setup(|app| {
            // Hide from dock on macOS because we don't need an icon lingering around for a tray app
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }

            // Enable autostart
            {
                if let Ok(false) = app.autolaunch().is_enabled() {
                    let _ = app.autolaunch().enable();
                }
            }

            // Create tray menu
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let settings_item = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&settings_item, &quit_item])?;

            let _tray = TrayIconBuilder::with_id("main")
                .title("📅")
                .tooltip("ZemenBar - Ethiopian Calendar")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "settings" => {
                        println!("Settings menu clicked!");
                        if let Some(window) = app.get_webview_window("settings") {
                            println!("Found settings window from menu");
                            let _ = window.show();
                            let _ = window.set_focus();
                        } else {
                            println!("Settings window not found from menu!");
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        position,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("settings") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                // So that it's under the tray text
                                if let Ok(monitor) = window.primary_monitor() {
                                    if let Some(monitor) = monitor {
                                        let scale = monitor.scale_factor();
                                        let x = position.x / scale - 170.0; // center under click (half of 340px width roughly)
                                        let y = 0.0;
                                        let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition { x, y }));
                                    }
                                }
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            // Click away -> hide
            if let Some(window) = app.get_webview_window("settings") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    match event {
                        tauri::WindowEvent::Focused(false) => {
                            if window_clone.is_visible().unwrap_or(false) {
                                let _ = window_clone.hide();
                            }
                        }
                        _ => {}
                    }
                });
            }

            // Init
            {
                let app_handle = app.app_handle().clone();
                let _ = update_tray_with_date(&app_handle);
                std::thread::spawn(move || {
                    loop {
                        let _ = update_tray_with_date(&app_handle);
                        std::thread::sleep(Duration::from_secs(3600));
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_tray_text,
            load_settings,
            save_settings,
            update_tray_display
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
