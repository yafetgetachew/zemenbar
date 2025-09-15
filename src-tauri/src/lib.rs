use ethiopic_calendar::{EthiopianYear, GregorianYear};
use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};

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

    pub fn days_in_month(&self) -> usize {
        if self.month == 13 {
            if self.year % 4 == 3 {
                6
            } else {
                5
            }
        } else {
            30
        }
    }

    pub fn weekday(&self) -> usize {
        // Convert back to Gregorian to get weekday
        let ethiopian = EthiopianYear::new(self.year, self.month, self.day);
        let gregorian: GregorianYear = ethiopian.into();

        // Create a chrono date to get weekday
        if let Some(date) = chrono::NaiveDate::from_ymd_opt(gregorian.year() as i32, gregorian.month() as u32, gregorian.day() as u32) {
            date.weekday().num_days_from_sunday() as usize
        } else {
            0
        }
    }

    pub fn amharic_weekday(&self) -> &'static str {
        match self.weekday() {
            0 => "እሁድ",
            1 => "ሰኞ",
            2 => "ማክሰኞ",
            3 => "ረቡዕ",
            4 => "ሐሙስ",
            5 => "ዓርብ",
            6 => "ቅዳሜ",
            _ => "Unknown",
        }
    }

    pub fn english_weekday(&self) -> &'static str {
        match self.weekday() {
            0 => "Sunday",
            1 => "Monday",
            2 => "Tuesday",
            3 => "Wednesday",
            4 => "Thursday",
            5 => "Friday",
            6 => "Saturday",
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

#[derive(Serialize, Deserialize)]
pub struct CalendarMonth {
    pub year: usize,
    pub year_geez: String,
    pub month: usize,
    pub month_name_amharic: String,
    pub month_name_english: String,
    pub days: Vec<CalendarDay>,
    pub first_day_weekday: usize,
}

#[derive(Serialize, Deserialize)]
pub struct CalendarDay {
    pub day: usize,
    pub day_geez: String,
    pub is_today: bool,
    pub weekday: usize,
    pub weekday_name_amharic: String,
    pub weekday_name_english: String,
}

impl CalendarMonth {
    pub fn new(year: usize, month: usize) -> Self {
        let first_day = EthiopianDate { year, month, day: 1, day_geez: EthiopianDate::to_geez_number(1) };
        let days_in_month = first_day.days_in_month();
        let first_day_weekday = first_day.weekday();
        let today = EthiopianDate::today();

        let mut days = Vec::new();
        for day in 1..=days_in_month {
            let date = EthiopianDate { year, month, day, day_geez: EthiopianDate::to_geez_number(day) };
            let is_today = date.year == today.year && date.month == today.month && date.day == today.day;
            days.push(CalendarDay {
                day,
                day_geez: date.day_geez(),
                is_today,
                weekday: date.weekday(),
                weekday_name_amharic: date.amharic_weekday().to_string(),
                weekday_name_english: date.english_weekday().to_string(),
            });
        }

        Self {
            year,
            year_geez: EthiopianDate::to_geez_number(year),
            month,
            month_name_amharic: first_day.amharic_month().to_string(),
            month_name_english: first_day.english_month().to_string(),
            days,
            first_day_weekday,
        }
    }
}
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};
use std::sync::Mutex;

// Global variable to store the last tray position
static LAST_TRAY_X: Mutex<Option<f64>> = Mutex::new(None);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub use_amharic: bool,
    pub use_geez_numbers: bool,
    pub show_date_in_tray: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            use_amharic: true,
            use_geez_numbers: false,
            show_date_in_tray: true,
        }
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_current_ethiopian_date() -> EthiopianDate {
    EthiopianDate::today()
}

#[tauri::command]
fn get_ethiopian_calendar_month(year: usize, month: usize) -> CalendarMonth {
    CalendarMonth::new(year, month)
}

#[tauri::command]
fn convert_gregorian_to_ethiopian(year: i32, month: u32, day: u32) -> Option<EthiopianDate> {
    EthiopianDate::from_gregorian(year, month, day)
}

#[tauri::command]
fn position_calendar_window(app: tauri::AppHandle, tray_x: Option<f64>) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        // Get the primary monitor to calculate proper positioning
        if let Ok(monitor) = window.primary_monitor() {
            if let Some(monitor) = monitor {
                let scale_factor = monitor.scale_factor();

                // Position window directly under the tray icon
                let x = if let Some(tray_x) = tray_x {
                    // Store the new tray position
                    if let Ok(mut last_x) = LAST_TRAY_X.lock() {
                        *last_x = Some(tray_x);
                    }
                    // Use provided tray position
                    tray_x / scale_factor
                } else {
                    // Try to use stored tray position first
                    if let Ok(last_x) = LAST_TRAY_X.lock() {
                        if let Some(stored_x) = *last_x {
                            stored_x / scale_factor
                        } else {
                            // Fallback: estimate tray position (usually in top-right area)
                            let size = monitor.size();
                            let logical_width = (size.width as f64) / scale_factor;
                            logical_width - 380.0 // 360px window + 20px margin
                        }
                    } else {
                        // Fallback: estimate tray position (usually in top-right area)
                        let size = monitor.size();
                        let logical_width = (size.width as f64) / scale_factor;
                        logical_width - 380.0 // 360px window + 20px margin
                    }
                };

                let y = 28.0; // Below menu bar
                let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition { x, y }));
            }
        }
    }
    Ok(())
}

#[tauri::command]
fn resize_calendar_window(app: tauri::AppHandle, height: f64) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize { width: 360.0, height }));
    }
    Ok(())
}

#[tauri::command]
fn set_tray_text(app: tauri::AppHandle, text: String) -> Result<(), String> {
    println!("Setting tray text to: {}", text);
    if let Some(tray) = app.tray_by_id("main") {
        // Just set the title, keep any existing icon
        let _ = tray.set_title(Some(&text));
        println!("Tray text set successfully");
    } else {
        println!("Tray not found!");
    }
    Ok(())
}

#[tauri::command]
fn set_tray_icon(app: tauri::AppHandle) -> Result<(), String> {
    println!("Setting tray to icon mode");
    if let Some(tray) = app.tray_by_id("main") {
        // Clear the title to show just the icon
        let _ = tray.set_title(Some("📅"));
        println!("Tray icon set successfully");
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
            // Hide from dock on macOS
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }

            // Enable autostart (login item) only if not already enabled
            {
                use tauri_plugin_autostart::ManagerExt;
                if let Ok(false) = app.autolaunch().is_enabled() {
                    let _ = app.autolaunch().enable();
                }
            }

            // Create tray menu
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_item = MenuItem::with_id(app, "show", "Show Calendar", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

            // Create system tray
            let _tray = TrayIconBuilder::with_id("main")
                .title("")
                .tooltip("ZemenBar - Ethiopian Calendar")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("settings") {
                            // TODO: Make Position of window fixed
                            let _ = position_calendar_window(app.clone(), None);
                            let _ = window.show();
                            let _ = window.set_focus();
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
                                // Position window directly under the tray icon
                                let tray_x = position.x - 180.0; // Center the 360px window under the tray icon (TODO: change to fixed)
                                let _ = position_calendar_window(app.clone(), Some(tray_x));
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            // Initialize tray text to today's date (default is always date text)
            {
                let settings = load_settings(app.handle().clone()).unwrap_or_default();
                let today = EthiopianDate::today();
                let month_meta = CalendarMonth::new(today.year, today.month);
                let month_name = if settings.use_amharic { month_meta.month_name_amharic.clone() } else { month_meta.month_name_english.clone() };
                let day_txt = if settings.use_geez_numbers { today.day_geez.clone() } else { today.day.to_string() };
                let year_txt = if settings.use_geez_numbers { month_meta.year_geez.clone() } else { today.year.to_string() };
                let text = format!("{} {} {}", month_name, day_txt, year_txt);
                if let Some(tray) = app.tray_by_id("main") {
                    let _ = tray.set_title(Some(&text));
                }
            }

            // Set up window event handler for click-away behavior
            if let Some(window) = app.get_webview_window("settings") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    match event {
                        tauri::WindowEvent::Focused(false) => {
                            // Hide window when it loses focus (click away) - but only if it's visible
                            if window_clone.is_visible().unwrap_or(false) {
                                let _ = window_clone.hide();
                            }
                        }
                        _ => {}
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_current_ethiopian_date,
            get_ethiopian_calendar_month,
            convert_gregorian_to_ethiopian,
            position_calendar_window,
            resize_calendar_window,
            set_tray_text,
            set_tray_icon,
            load_settings,
            save_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}