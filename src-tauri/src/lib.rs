//!
//! This library provides Ethiopian calendar functionality for Zemenbar with system tray integration.

use chrono::{Datelike, Duration, FixedOffset, NaiveDate, Utc};
use ethiopic_calendar::{EthiopianYear, GregorianYear};
use hijri_date::HijriDate;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};
use tauri_nspanel::{tauri_panel, CollectionBehavior, PanelLevel, StyleMask, WebviewWindowExt};

/// Represents a date in the Ethiopian calendar system.
///
/// The Ethiopian calendar has 13 months: 12 months of 30 days each,
/// plus Pagumē with 5 or 6 days depending on leap years.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthiopianDate {
    pub year: usize,
    pub month: usize,
    pub day: usize,
    /// Day formatted in Geez numerals
    pub day_geez: String,
}

impl EthiopianDate {
    /// Creates an `EthiopianDate` representing today's date in EAT (GMT+3).
    pub fn today() -> Self {
        let eat_offset = FixedOffset::east_opt(3 * 3600).unwrap();
        let today = Utc::now().with_timezone(&eat_offset).date_naive();
        let gregorian = GregorianYear::new(
            today.year() as usize,
            today.month() as usize,
            today.day() as usize,
        );
        let ethiopian: EthiopianYear = gregorian.into();

        let day = ethiopian.day();
        Self {
            year: ethiopian.year(),
            month: ethiopian.month(),
            day,
            day_geez: Self::to_geez_number(day),
        }
    }

    /// Converts a Gregorian date to Ethiopian calendar.
    ///
    /// Returns `None` if the conversion fails.
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
        let ethiopian = EthiopianYear::new(self.year, self.month, self.day);
        let gregorian: GregorianYear = ethiopian.into();

        if let Some(date) = chrono::NaiveDate::from_ymd_opt(
            gregorian.year() as i32,
            gregorian.month() as u32,
            gregorian.day() as u32,
        ) {
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

    /// Converts Arabic numerals to Geez numerals.
    pub fn to_geez_number(num: usize) -> String {
        if num == 0 {
            return "".to_string();
        }

        let geez_digits = ["", "፩", "፪", "፫", "፬", "፭", "፮", "፯", "፰", "፱"];
        let geez_tens = ["", "፲", "፳", "፴", "፵", "፶", "፷", "፸", "፹", "፺"];

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
            } else if ones == 0 {
                geez_tens[tens].to_string()
            } else {
                format!("{}{}", geez_tens[tens], geez_digits[ones])
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
            let thousands = num / 100;
            let remainder = num % 100;

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
                } else if ones == 0 {
                    format!("{}፻", geez_tens[tens])
                } else {
                    format!("{}{}፻", geez_tens[tens], geez_digits[ones])
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

/// Represents a complete month view for the Ethiopian calendar.
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
    pub is_holiday: bool,
    pub weekday: usize,
    pub weekday_name_amharic: String,
    pub weekday_name_english: String,
    pub special_days: Vec<SpecialDay>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialDay {
    pub name_amharic: String,
    pub name_english: String,
    pub category: String,
}

fn special_day(name_amharic: &str, name_english: &str, category: &str) -> SpecialDay {
    SpecialDay {
        name_amharic: name_amharic.to_string(),
        name_english: name_english.to_string(),
        category: category.to_string(),
    }
}

fn fixed_ethiopian_special_days(month: usize, day: usize) -> Vec<SpecialDay> {
    match (month, day) {
        (1, 1) => vec![special_day("እንቁጣጣሽ", "Enkutatash (New Year)", "national")],
        (1, 17) => vec![special_day("መስቀል", "Meskel", "religious-christian")],
        (3, 29) => vec![special_day(
            "የብሔር ብሔረሰቦች እና ሕዝቦች ቀን",
            "Nations, Nationalities and Peoples' Day",
            "government",
        )],
        (4, 29) => vec![special_day(
            "ገና",
            "Genna (Christmas)",
            "religious-christian",
        )],
        (5, 11) => vec![special_day(
            "ጥምቀት",
            "Timket (Epiphany)",
            "religious-christian",
        )],
        (6, 23) => vec![special_day("የአድዋ ድል ቀን", "Adwa Victory Day", "government")],
        (8, 23) => vec![special_day(
            "የሰራተኞች ቀን",
            "International Workers' Day",
            "government",
        )],
        (8, 27) => vec![special_day(
            "የአርበኞች የድል ቀን",
            "Patriots' Victory Day",
            "government",
        )],
        (9, 20) => vec![special_day("ግንቦት 20", "Derg Downfall Day", "government")],
        _ => Vec::new(),
    }
}

type HijriHolidayDefinition = ((u8, u8), (&'static str, &'static str));

const MUSLIM_HOLIDAY_DEFINITIONS: [HijriHolidayDefinition; 5] = [
    ((1, 1), ("አዲስ ዓመተ ሂጅራ", "Islamic New Year")),
    ((3, 12), ("መውሊድ", "Mawlid al-Nabi")),
    ((10, 1), ("ኢድ አልፊጥር", "Eid al-Fitr")),
    ((12, 9), ("የአረፋ ቀን", "Arafa")),
    ((12, 10), ("ኢድ አልአድሃ", "Eid al-Adha")),
];

fn muslim_special_days_for_ethiopian_year(
    ethiopian_year: usize,
) -> HashMap<(usize, usize), Vec<SpecialDay>> {
    let mut indexed_days: HashMap<(usize, usize), Vec<SpecialDay>> = HashMap::new();

    let gregorian_start_year = ethiopian_year + 7;
    let gregorian_end_year = ethiopian_year + 8;
    let approx_hijri_year = gregorian_start_year.saturating_sub(579);

    for hijri_year in approx_hijri_year.saturating_sub(2)..=approx_hijri_year + 2 {
        for ((hijri_month, hijri_day), (name_amharic, name_english)) in MUSLIM_HOLIDAY_DEFINITIONS {
            let Ok(hijri_date) =
                HijriDate::from_hijri(hijri_year, hijri_month as usize, hijri_day as usize)
            else {
                continue;
            };
            let gregorian_year = hijri_date.year_gr();

            if gregorian_year != gregorian_start_year && gregorian_year != gregorian_end_year {
                continue;
            }

            if let Some(ethiopian_date) = EthiopianDate::from_gregorian(
                gregorian_year as i32,
                hijri_date.month_gr() as u32,
                hijri_date.day_gr() as u32,
            ) {
                if ethiopian_date.year != ethiopian_year {
                    continue;
                }

                indexed_days
                    .entry((ethiopian_date.month, ethiopian_date.day))
                    .or_default()
                    .push(special_day(name_amharic, name_english, "religious-muslim"));
            }
        }
    }

    for special_days in indexed_days.values_mut() {
        special_days.sort_by(|a, b| a.name_english.cmp(&b.name_english));
        special_days.dedup_by(|a, b| a.name_english == b.name_english);
    }

    indexed_days
}

fn orthodox_easter_gregorian(gregorian_year: i32) -> Option<NaiveDate> {
    let a = gregorian_year.rem_euclid(4);
    let b = gregorian_year.rem_euclid(7);
    let c = gregorian_year.rem_euclid(19);
    let d = (19 * c + 15).rem_euclid(30);
    let e = (2 * a + 4 * b - d + 34).rem_euclid(7);

    let julian_month = ((d + e + 114) / 31) as u32;
    let julian_day = ((d + e + 114) % 31 + 1) as u32;

    let julian_easter = NaiveDate::from_ymd_opt(gregorian_year, julian_month, julian_day)?;
    let gregorian_shift_days = (gregorian_year / 100) - (gregorian_year / 400) - 2;
    Some(julian_easter + Duration::days(gregorian_shift_days as i64))
}

fn christian_movable_special_days_for_ethiopian_year(
    ethiopian_year: usize,
) -> HashMap<(usize, usize), Vec<SpecialDay>> {
    let mut indexed_days: HashMap<(usize, usize), Vec<SpecialDay>> = HashMap::new();

    let gregorian_year = (ethiopian_year + 8) as i32;
    let Some(easter) = orthodox_easter_gregorian(gregorian_year) else {
        return indexed_days;
    };

    let observances = [
        (easter - Duration::days(7), "ሆሳዕና", "Hosanna (Palm Sunday)"),
        (easter - Duration::days(2), "ስቅለት", "Siqlet (Good Friday)"),
        (easter, "ፋሲካ", "Fasika (Easter)"),
        (easter + Duration::days(39), "ዕርገት", "Erget (Ascension)"),
        (
            easter + Duration::days(49),
            "ጰራቅሊጦስ",
            "Peraklitos (Pentecost)",
        ),
    ];

    for (gregorian_date, name_amharic, name_english) in observances {
        if let Some(ethiopian_date) = EthiopianDate::from_gregorian(
            gregorian_date.year(),
            gregorian_date.month(),
            gregorian_date.day(),
        ) {
            if ethiopian_date.year != ethiopian_year {
                continue;
            }

            indexed_days
                .entry((ethiopian_date.month, ethiopian_date.day))
                .or_default()
                .push(special_day(
                    name_amharic,
                    name_english,
                    "religious-christian",
                ));
        }
    }

    indexed_days
}

impl CalendarMonth {
    pub fn new(year: usize, month: usize) -> Self {
        let first_day = EthiopianDate {
            year,
            month,
            day: 1,
            day_geez: EthiopianDate::to_geez_number(1),
        };
        let days_in_month = first_day.days_in_month();
        let first_day_weekday = first_day.weekday();
        let today = EthiopianDate::today();
        let muslim_special_days = muslim_special_days_for_ethiopian_year(year);
        let christian_movable_special_days =
            christian_movable_special_days_for_ethiopian_year(year);

        let mut days = Vec::new();
        for day in 1..=days_in_month {
            let date = EthiopianDate {
                year,
                month,
                day,
                day_geez: EthiopianDate::to_geez_number(day),
            };
            let is_today =
                date.year == today.year && date.month == today.month && date.day == today.day;
            let mut special_days = fixed_ethiopian_special_days(month, day);
            if let Some(dynamic_days) = muslim_special_days.get(&(month, day)) {
                special_days.extend(dynamic_days.iter().cloned());
            }
            if let Some(dynamic_days) = christian_movable_special_days.get(&(month, day)) {
                special_days.extend(dynamic_days.iter().cloned());
            }

            days.push(CalendarDay {
                day,
                day_geez: date.day_geez(),
                is_today,
                is_holiday: !special_days.is_empty(),
                weekday: date.weekday(),
                weekday_name_amharic: date.amharic_weekday().to_string(),
                weekday_name_english: date.english_weekday().to_string(),
                special_days,
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

tauri_panel! {
    panel!(CalendarPanel {
        config: {
            can_become_key_window: true,
            is_floating_panel: true
        }
    })
}

/// Global state for remembering tray icon position to position calendar window correctly.
static LAST_TRAY_X: Mutex<Option<f64>> = Mutex::new(None);

/// Application settings that control calendar display and behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub use_amharic: bool,
    pub use_geez_numbers: bool,
    pub show_date_in_tray: bool,
    pub use_numeric_format: bool,
    pub show_qen: bool,
    pub show_amete_mihret: bool,
    pub show_national_special_days: bool,
    pub show_government_special_days: bool,
    pub show_christian_special_days: bool,
    pub show_muslim_special_days: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            use_amharic: true,
            use_geez_numbers: false,
            show_date_in_tray: true,
            use_numeric_format: false,
            show_qen: false,
            show_amete_mihret: false,
            show_national_special_days: false,
            show_government_special_days: false,
            show_christian_special_days: false,
            show_muslim_special_days: false,
        }
    }
}

#[tauri::command]
fn get_current_ethiopian_date() -> EthiopianDate {
    EthiopianDate::today()
}

#[tauri::command]
fn get_ethiopian_calendar_month(year: usize, month: usize) -> CalendarMonth {
    CalendarMonth::new(year, month)
}

/// Tauri command to convert Gregorian date to Ethiopian calendar.
#[tauri::command]
fn convert_gregorian_to_ethiopian(year: i32, month: u32, day: u32) -> Option<EthiopianDate> {
    EthiopianDate::from_gregorian(year, month, day)
}

/// Positions the calendar window relative to the tray icon. Maybe it would be to have it left align to tray? TODO
#[tauri::command]
fn position_calendar_window(app: tauri::AppHandle, tray_x: Option<f64>) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        if let Ok(Some(monitor)) = window.primary_monitor() {
            let scale_factor = monitor.scale_factor();

            let x = if let Some(tray_x) = tray_x {
                if let Ok(mut last_x) = LAST_TRAY_X.lock() {
                    *last_x = Some(tray_x);
                }
                tray_x / scale_factor
            } else if let Ok(last_x) = LAST_TRAY_X.lock() {
                if let Some(stored_x) = *last_x {
                    stored_x / scale_factor
                } else {
                    let size = monitor.size();
                    let logical_width = (size.width as f64) / scale_factor;
                    logical_width - 380.0
                }
            } else {
                let size = monitor.size();
                let logical_width = (size.width as f64) / scale_factor;
                logical_width - 380.0
            };

            let y = 28.0;
            let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition { x, y }));
        }
    }
    Ok(())
}

#[tauri::command]
fn resize_calendar_window(app: tauri::AppHandle, height: f64) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize {
            width: 360.0,
            height,
        }));
    }
    Ok(())
}

#[tauri::command]
fn set_tray_text(app: tauri::AppHandle, text: String) -> Result<(), String> {
    if let Some(tray) = app.tray_by_id("main") {
        let _ = tray.set_title(Some(&text));
    }
    Ok(())
}

#[tauri::command]
fn set_tray_icon(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(tray) = app.tray_by_id("main") {
        let _ = tray.set_title(Some("📅"));
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

        serde_json::from_str(&content).map_err(|e| format!("Failed to parse settings: {}", e))
    } else {
        Ok(AppSettings::default())
    }
}

/// Copies text to the system clipboard.
#[tauri::command]
async fn copy_to_clipboard(app: tauri::AppHandle, text: String) -> Result<(), String> {
    use tauri_plugin_clipboard_manager::ClipboardExt;

    app.clipboard()
        .write_text(text)
        .map_err(|e| format!("Failed to copy to clipboard: {}", e))
}

#[tauri::command]
fn save_settings(app: tauri::AppHandle, settings: AppSettings) -> Result<(), String> {
    let settings_path = get_settings_path(&app)?;

    if let Some(parent) = settings_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create settings directory: {}", e))?;
    }

    let content = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    std::fs::write(&settings_path, content)
        .map_err(|e| format!("Failed to write settings file: {}", e))?;
    Ok(())
}

#[tauri::command]
fn refresh_tray_display(app: tauri::AppHandle) -> Result<(), String> {
    let settings = load_settings(app.clone()).unwrap_or_default();
    let today = EthiopianDate::today();
    let month_meta = CalendarMonth::new(today.year, today.month);

    let text = if settings.use_numeric_format {
        let dd = if settings.use_geez_numbers {
            EthiopianDate::to_geez_number(today.day)
        } else {
            format!("{:02}", today.day)
        };
        let mm = if settings.use_geez_numbers {
            EthiopianDate::to_geez_number(today.month)
        } else {
            format!("{:02}", today.month)
        };
        let yyyy = if settings.use_geez_numbers {
            EthiopianDate::to_geez_number(today.year)
        } else {
            today.year.to_string()
        };

        let parts = [dd, mm, yyyy];
        parts.join("/")
    } else {
        let month_name = if settings.use_amharic {
            month_meta.month_name_amharic.clone()
        } else {
            month_meta.month_name_english.clone()
        };
        let day_txt = if settings.use_geez_numbers {
            today.day_geez.clone()
        } else {
            today.day.to_string()
        };
        let year_txt = if settings.use_geez_numbers {
            month_meta.year_geez.clone()
        } else {
            today.year.to_string()
        };

        let mut parts = Vec::new();
        parts.push(month_name);
        parts.push(day_txt);
        if settings.use_amharic && settings.show_qen {
            parts.push("ቀን".to_string());
        }
        parts.push(year_txt);
        if settings.use_amharic && settings.show_amete_mihret {
            parts.push("ዓ.ም.".to_string());
        }
        parts.join(" ")
    };

    if let Some(tray) = app.tray_by_id("main") {
        let _ = tray.set_title(Some(&text));
    }

    Ok(())
}

fn create_calendar_panel(app: &tauri::App) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        let panel = window
            .to_panel::<CalendarPanel>()
            .map_err(|e| format!("Failed to convert window to panel: {}", e))?;

        panel.set_level(PanelLevel::Floating.value());
        panel.set_style_mask(StyleMask::empty().nonactivating_panel().into());
        panel.set_collection_behavior(
            CollectionBehavior::new()
                .full_screen_auxiliary()
                .can_join_all_spaces()
                .into(),
        );
        panel.set_hides_on_deactivate(false);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geez_tens_are_mapped_correctly() {
        assert_eq!(EthiopianDate::to_geez_number(70), "፸");
        assert_eq!(EthiopianDate::to_geez_number(80), "፹");
        assert_eq!(EthiopianDate::to_geez_number(90), "፺");
    }

    #[test]
    fn fixed_holiday_lookup_returns_expected_days() {
        let new_year = fixed_ethiopian_special_days(1, 1);
        assert!(new_year
            .iter()
            .any(|day| day.name_english == "Enkutatash (New Year)"));

        let christmas = fixed_ethiopian_special_days(4, 29);
        assert!(christmas
            .iter()
            .any(|day| day.name_english == "Genna (Christmas)"));
    }

    #[test]
    fn fixed_holiday_lookup_returns_empty_for_normal_day() {
        assert!(fixed_ethiopian_special_days(2, 12).is_empty());
    }

    #[test]
    fn muslim_holidays_are_projected_for_ethiopian_year() {
        let projected = muslim_special_days_for_ethiopian_year(2018);
        assert!(!projected.is_empty());

        let all_days = projected.values().flatten().collect::<Vec<_>>();
        assert!(
            all_days.iter().any(|day| day.name_english == "Eid al-Fitr"),
            "Expected Eid al-Fitr in projected Muslim observances"
        );
        assert!(
            all_days.iter().any(|day| day.name_english == "Eid al-Adha"),
            "Expected Eid al-Adha in projected Muslim observances"
        );
    }

    #[test]
    fn orthodox_easter_related_holidays_are_projected() {
        let projected = christian_movable_special_days_for_ethiopian_year(2018);
        assert!(!projected.is_empty());

        let all_days = projected.values().flatten().collect::<Vec<_>>();
        assert!(
            all_days
                .iter()
                .any(|day| day.name_english == "Fasika (Easter)"),
            "Expected Fasika (Easter) in projected Christian observances"
        );
        assert!(
            all_days
                .iter()
                .any(|day| day.name_english == "Siqlet (Good Friday)"),
            "Expected Siqlet (Good Friday) in projected Christian observances"
        );
    }

    #[test]
    fn special_day_category_toggles_default_to_off() {
        let defaults = AppSettings::default();
        assert!(!defaults.show_national_special_days);
        assert!(!defaults.show_government_special_days);
        assert!(!defaults.show_christian_special_days);
        assert!(!defaults.show_muslim_special_days);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_nspanel::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(window) = app.get_webview_window("settings") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }

            {
                use tauri_plugin_autostart::ManagerExt;
                if let Ok(false) = app.autolaunch().is_enabled() {
                    let _ = app.autolaunch().enable();
                }
            }
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
                            let _ = position_calendar_window(app.clone(), None);
                            let _ = window.show();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        position,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("settings") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let tray_x = position.x - 180.0;
                                let _ = position_calendar_window(app.clone(), Some(tray_x));
                                let _ = window.show();
                            }
                        }
                    }
                })
                .build(app)?;
            {
                let settings = load_settings(app.handle().clone()).unwrap_or_default();
                let today = EthiopianDate::today();
                let month_meta = CalendarMonth::new(today.year, today.month);
                let month_name = if settings.use_amharic {
                    month_meta.month_name_amharic.clone()
                } else {
                    month_meta.month_name_english.clone()
                };
                let day_txt = if settings.use_geez_numbers {
                    today.day_geez.clone()
                } else {
                    today.day.to_string()
                };
                let year_txt = if settings.use_geez_numbers {
                    month_meta.year_geez.clone()
                } else {
                    today.year.to_string()
                };
                let text = format!("{} {} {}", month_name, day_txt, year_txt);
                if let Some(tray) = app.tray_by_id("main") {
                    let _ = tray.set_title(Some(&text));
                }
            }
            if let Some(window) = app.get_webview_window("settings") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::Focused(false) = event {
                        if window_clone.is_visible().unwrap_or(false) {
                            let _ = window_clone.hide();
                        }
                    }
                });
            }

            if let Err(e) = create_calendar_panel(app) {
                eprintln!("Failed to setup calendar panel: {}", e);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_current_ethiopian_date,
            get_ethiopian_calendar_month,
            convert_gregorian_to_ethiopian,
            position_calendar_window,
            resize_calendar_window,
            set_tray_text,
            set_tray_icon,
            load_settings,
            save_settings,
            copy_to_clipboard,
            refresh_tray_display
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
