<!--
  Ethiopian Calendar Component

  Main calendar interface with month navigation and settings controls.
  Manages tray text updates and window positioning.
-->

<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  // Removed direct clipboard import - using Tauri command instead

  interface EthiopianDate {
    year: number;
    month: number;
    day: number;
    day_geez: string;
  }

  interface AppSettings {
    use_amharic: boolean;
    use_geez_numbers: boolean;
    show_date_in_tray: boolean;
    use_numeric_format: boolean;
    show_qen: boolean;
    show_amete_mihret: boolean;
    show_national_special_days: boolean;
    show_government_special_days: boolean;
    show_christian_special_days: boolean;
    show_muslim_special_days: boolean;
  }

  interface SpecialDay {
    name_amharic: string;
    name_english: string;
    category: string;
  }

  interface CalendarDay {
    day: number;
    day_geez: string;
    is_today: boolean;
    is_holiday: boolean;
    weekday: number;
    weekday_name_amharic: string;
    weekday_name_english: string;
    special_days: SpecialDay[];
  }

  interface MonthSpecialDay {
    day: CalendarDay;
    specialDay: SpecialDay;
  }

  interface CalendarMonth {
    year: number;
    year_geez: string;
    month: number;
    month_name_amharic: string;
    month_name_english: string;
    days: CalendarDay[];
    first_day_weekday: number;
  }

  // State vars
  let currentDate: EthiopianDate | null = $state(null);
  let calendarMonth: CalendarMonth | null = $state(null);
  let todayMonthMeta: CalendarMonth | null = $state(null);

  // Display state
  let displayYear = $state(0);
  let displayMonth = $state(0);

  // Settings state
  let useAmharic = $state(true);
  let useGeezNumbers = $state(false);
  let showDateInTray = $state(true);
  let useNumericFormat = $state(false);
  let showQen = $state(false);
  let showAmeteMihret = $state(false);
  let showNationalSpecialDays = $state(false);
  let showGovernmentSpecialDays = $state(false);
  let showChristianSpecialDays = $state(false);
  let showMuslimSpecialDays = $state(false);
  let showSpecialDayOptions = $state(false);

  /**
   * Loads the current Ethiopian date from the backend.
   */
  async function loadCurrentDate(syncDisplayMonth = true) {
    try {
      currentDate = await invoke<EthiopianDate>("get_current_ethiopian_date");

      todayMonthMeta = await invoke<CalendarMonth>("get_ethiopian_calendar_month", {
        year: currentDate.year,
        month: currentDate.month,
      });

      if (syncDisplayMonth || displayYear === 0 || displayMonth === 0) {
        displayYear = currentDate.year;
        displayMonth = currentDate.month;
        await loadCalendarMonth();
        return;
      }

      if (displayYear === currentDate.year && displayMonth === currentDate.month) {
        await loadCalendarMonth();
      }
    } catch (error) {
      console.error("Failed to load current date:", error);
    }
  }

  async function loadCalendarMonth() {
    try {
      calendarMonth = await invoke<CalendarMonth>("get_ethiopian_calendar_month", {
        year: displayYear,
        month: displayMonth,
      });

      if (calendarMonth) {
        await resizeWindowForContent();
      }
    } catch (error) {
      console.error("Failed to load calendar month:", error);
    }
  }

  async function resizeWindowForContent() {
    if (!calendarMonth) return;

    try {
      const baseHeight = 210 + (showSpecialDayOptions ? 42 : 0);
      const weekdayHeaderHeight = 40;
      const dayRowHeight = 42;
      const totalDays = calendarMonth.days.length;
      const emptyDays = calendarMonth.first_day_weekday;
      const totalCells = totalDays + emptyDays;
      const rows = Math.ceil(totalCells / 7);
      const calendarGridHeight = weekdayHeaderHeight + (rows * dayRowHeight);
      const monthSpecialDayCount = calendarMonth.days.reduce(
        (total, day) => total + getVisibleSpecialDaysForDay(day).length,
        0
      );
      const specialDaysHeight = monthSpecialDayCount > 0 ? 34 + (monthSpecialDayCount * 22) : 0;
      const footerHeight = 60;

      const totalHeight = baseHeight + calendarGridHeight + footerHeight + specialDaysHeight;

      await invoke("resize_calendar_window", { height: totalHeight });
      await invoke("position_calendar_window", { trayX: null });
    } catch (error) {
      console.error("Failed to resize window:", error);
    }
  }

  async function previousMonth() {
    if (displayMonth === 1) {
      displayMonth = 13;
      displayYear--;
    } else {
      displayMonth--;
    }
    await loadCalendarMonth();
    updateTrayDisplay();
  }

  async function nextMonth() {
    if (displayMonth === 13) {
      displayMonth = 1;
      displayYear++;
    } else {
      displayMonth++;
    }
    await loadCalendarMonth();
    updateTrayDisplay();
  }

  async function goToToday() {
    if (currentDate) {
      displayYear = currentDate.year;
      displayMonth = currentDate.month;
      await loadCalendarMonth();
      updateTrayDisplay();
    }
  }

  async function loadSettings() {
    try {
      const settings: AppSettings = await invoke("load_settings");
      useAmharic = settings.use_amharic;
      useGeezNumbers = settings.use_geez_numbers;
      showDateInTray = settings.show_date_in_tray;
      useNumericFormat = settings.use_numeric_format;
      showQen = settings.show_qen;
      showAmeteMihret = settings.show_amete_mihret;
      showNationalSpecialDays = settings.show_national_special_days;
      showGovernmentSpecialDays = settings.show_government_special_days;
      showChristianSpecialDays = settings.show_christian_special_days;
      showMuslimSpecialDays = settings.show_muslim_special_days;
    } catch (error) {
      console.error("Failed to load settings:", error);
    }
  }

  async function saveSettings() {
    try {
      const settings: AppSettings = {
        use_amharic: useAmharic,
        use_geez_numbers: useGeezNumbers,
        show_date_in_tray: showDateInTray,
        use_numeric_format: useNumericFormat,
        show_qen: showQen,
        show_amete_mihret: showAmeteMihret,
        show_national_special_days: showNationalSpecialDays,
        show_government_special_days: showGovernmentSpecialDays,
        show_christian_special_days: showChristianSpecialDays,
        show_muslim_special_days: showMuslimSpecialDays,
      };
      await invoke("save_settings", { settings });
    } catch (error) {
      console.error("Failed to save settings:", error);
    }
  }

  async function toggleLanguage() {
    useAmharic = !useAmharic;
    await saveSettings();
    updateTrayDisplay();
  }

  async function toggleNumbers() {
    useGeezNumbers = !useGeezNumbers;
    await saveSettings();
    updateTrayDisplay();
  }

  async function toggleNumericFormat() {
    useNumericFormat = !useNumericFormat;
    await saveSettings();
    updateTrayDisplay();
  }

  async function toggleShowQen() {
    showQen = !showQen;
    await saveSettings();
    updateTrayDisplay();
  }

  async function toggleShowAmeteMihret() {
    showAmeteMihret = !showAmeteMihret;
    await saveSettings();
    updateTrayDisplay();
  }

  async function toggleNationalSpecialDays() {
    showNationalSpecialDays = !showNationalSpecialDays;
    await saveSettings();
    await resizeWindowForContent();
  }

  async function toggleGovernmentSpecialDays() {
    showGovernmentSpecialDays = !showGovernmentSpecialDays;
    await saveSettings();
    await resizeWindowForContent();
  }

  async function toggleChristianSpecialDays() {
    showChristianSpecialDays = !showChristianSpecialDays;
    await saveSettings();
    await resizeWindowForContent();
  }

  async function toggleMuslimSpecialDays() {
    showMuslimSpecialDays = !showMuslimSpecialDays;
    await saveSettings();
    await resizeWindowForContent();
  }

  async function toggleSpecialDayOptions() {
    showSpecialDayOptions = !showSpecialDayOptions;
    await resizeWindowForContent();
  }

  /**
   * Measures text width in pixels using Canvas API for tray text fitting.
   */
  function measureTextWidth(text: string, font = "13px -apple-system, system-ui, Segoe UI, Roboto"): number {
    const canvas = document.createElement("canvas");
    const ctx = canvas.getContext("2d");
    if (!ctx) return text.length * 8;
    ctx.font = font;
    return ctx.measureText(text).width;
  }

  /**
   * Copies the current date text to the clipboard.
   */
  async function copyDateToClipboard() {
    const dateText = getTodayDateDisplay();
    const button = document.querySelector('.copy-button');

    const showSuccess = () => {
      if (button) {
        const originalText = button.textContent;
        button.textContent = '✓';
        setTimeout(() => (button.textContent = originalText), 900);
      }
    };

    const showError = (e: unknown) => {
      console.error('Copy failed', e);
      if (button) {
        const originalText = button.textContent;
        button.textContent = '✗';
        setTimeout(() => (button.textContent = originalText), 900);
      }
    };

    try {
      // Use Tauri command for clipboard access
      await invoke('copy_to_clipboard', { text: dateText });
      console.log('Successfully copied to clipboard:', dateText);
      showSuccess();
    } catch (error) {
      console.error('Failed to copy to clipboard:', error);
      showError(error);
    }
  }

  function getTodayDayDisplay(): string {

    if (calendarMonth) {
      const t = calendarMonth.days.find((d) => d.is_today);
      if (t) return useGeezNumbers ? t.day_geez : t.day.toString();
    }
    return currentDate ? (useGeezNumbers ? currentDate.day_geez : currentDate.day.toString()) : "";
  }

  /**
   * Updates system tray text based on current date and settings.
   * Uses backend refresh command for consistency.
   */
  async function updateTrayDisplay() {
    try {
      await invoke("refresh_tray_display");
    } catch (error) {
      console.error("Failed to refresh tray display:", error);
      // Fallback to old method if backend fails
      await updateTrayDisplayFallback();
    }
  }

  /**
   * Fallback tray display update method.
   * Handles multiple formats and automatic text shortening.
   */
  async function updateTrayDisplayFallback() {
    if (!currentDate) {
      return;
    }

    try {
      const todayMeta = await invoke<CalendarMonth>("get_ethiopian_calendar_month", {
        year: currentDate.year,
        month: currentDate.month,
      });

      let textToShow: string;

      if (useNumericFormat) {
        const dd = currentDate.day.toString().padStart(2, '0');
        const mm = currentDate.month.toString().padStart(2, '0');
        const yyyy = currentDate.year.toString();
        textToShow = `${dd}/${mm}/${yyyy}`;
      } else {
        const monthName = useAmharic ? todayMeta.month_name_amharic : todayMeta.month_name_english;
        const day = useGeezNumbers ? currentDate.day_geez : currentDate.day.toString();
        const year = useGeezNumbers ? todayMeta.year_geez : currentDate.year.toString();

        let fullText = `${monthName} ${day} ${year}`;
        if (useAmharic) {
          if (showQen) {
            fullText = `${monthName} ${day} ቀን ${year}`;
          }
          if (showAmeteMihret) {
            fullText = `${fullText} ዓ.ም.`;
          }
        }

        const monthAbbrev = useAmharic ? monthName.slice(0, 2) : monthName.slice(0, 3);
        const compactText = `${monthAbbrev} ${day}`;

        const thresholdPx = 200;
        textToShow = measureTextWidth(fullText) <= thresholdPx ? fullText : compactText;
      }

      await invoke("set_tray_text", { text: textToShow });
    } catch (error) {
      console.error("Failed to update tray display:", error);
    }
  }

  function getDisplayNumber(day: CalendarDay): string {
    return useGeezNumbers ? day.day_geez : day.day.toString();
  }

  function isSpecialDayCategoryEnabled(category: string): boolean {
    switch (category) {
      case "national":
        return showNationalSpecialDays;
      case "government":
        return showGovernmentSpecialDays;
      case "religious-christian":
        return showChristianSpecialDays;
      case "religious-muslim":
        return showMuslimSpecialDays;
      default:
        return false;
    }
  }

  function getVisibleSpecialDaysForDay(day: CalendarDay): SpecialDay[] {
    return day.special_days.filter((specialDay) =>
      isSpecialDayCategoryEnabled(specialDay.category)
    );
  }

  function dayHasVisibleSpecialDay(day: CalendarDay): boolean {
    return getVisibleSpecialDaysForDay(day).length > 0;
  }

  function getSpecialDayDisplayName(specialDay: SpecialDay): string {
    return useAmharic ? specialDay.name_amharic : specialDay.name_english;
  }

  function getSpecialDayTooltip(day: CalendarDay): string {
    const visibleSpecialDays = getVisibleSpecialDaysForDay(day);
    if (visibleSpecialDays.length === 0) {
      return "";
    }

    return visibleSpecialDays
      .map((specialDay) => getSpecialDayDisplayName(specialDay))
      .join(" • ");
  }

  function getMonthSpecialDays(): MonthSpecialDay[] {
    if (!calendarMonth) {
      return [];
    }

    return calendarMonth.days.flatMap((day) =>
      getVisibleSpecialDaysForDay(day).map((specialDay) => ({ day, specialDay }))
    );
  }

  function getDisplayYear(): string {
    if (!calendarMonth) return "";
    return useGeezNumbers ? calendarMonth.year_geez : calendarMonth.year.toString();
  }

  function getTodayMonthName(): string {
    if (todayMonthMeta) {
      return useAmharic ? todayMonthMeta.month_name_amharic : todayMonthMeta.month_name_english;
    }
    return "";
  }

  function getTodayYearDisplay(): string {
    if (todayMonthMeta) {
      return useGeezNumbers
        ? todayMonthMeta.year_geez
        : (currentDate ? currentDate.year.toString() : "");
    }
    return currentDate ? currentDate.year.toString() : "";
  }

  function getTodayDateDisplay(): string {
    if (!currentDate) return "";

    if (useNumericFormat) {
      const dd = currentDate.day.toString().padStart(2, '0');
      const mm = currentDate.month.toString().padStart(2, '0');
      const yyyy = currentDate.year.toString();
      return `${dd}/${mm}/${yyyy}`;
    } else {
      let text = `${getTodayMonthName()} ${getTodayDayDisplay()} ${getTodayYearDisplay()}`;
      if (useAmharic) {
        if (showQen) {
          text = `${getTodayMonthName()} ${getTodayDayDisplay()} ቀን ${getTodayYearDisplay()}`;
        }
        if (showAmeteMihret) {
          text = `${text} ዓ.ም.`;
        }
      }

      return text;
    }
  }

  const monthSpecialDays = $derived(getMonthSpecialDays());

  onMount(() => {
    let refreshInterval: ReturnType<typeof setInterval> | null = null;

    const handleVisibilityChange = async () => {
      if (!document.hidden) {
        await loadCurrentDate(false);
        updateTrayDisplay();
      }
    };

    const initialize = async () => {
      await loadSettings();
      await loadCurrentDate();

      try {
        await invoke("position_calendar_window", { trayX: null });
        updateTrayDisplay();
      } catch (error) {
        console.error("Failed to position window:", error);
      }

      refreshInterval = setInterval(async () => {
        await loadCurrentDate(false);
        updateTrayDisplay();
      }, 10 * 60 * 1000);
    };

    document.addEventListener("visibilitychange", handleVisibilityChange);
    void initialize();

    return () => {
      if (refreshInterval) {
        clearInterval(refreshInterval);
      }
      document.removeEventListener("visibilitychange", handleVisibilityChange);
    };
  });

  const emptyStartCells = $derived(calendarMonth ? Array((calendarMonth as CalendarMonth).first_day_weekday).fill(null) : []);
  const weekdaysEnglish = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
  const weekdaysAmharic = ["እሁድ", "ሰኞ", "ማክሰኞ", "ረቡዕ", "ሐሙስ", "ዓርብ", "ቅዳሜ"];
</script>

<div class="calendar-container">
  {#if calendarMonth}
    <div class="calendar-header">
      <button class="nav-button" onclick={previousMonth}>‹</button>
      <div class="month-year">
        <h2 class="month-name">
          {useAmharic ? calendarMonth.month_name_amharic : calendarMonth.month_name_english}
        </h2>
        <div class="year">{getDisplayYear()}</div>
      </div>
      <button class="nav-button" onclick={nextMonth}>›</button>
    </div>

    <div class="calendar-controls">
      <button class="control-button" onclick={goToToday}>
        {useAmharic ? "ዛሬ" : "Today"}
      </button>
      <button class="control-button" onclick={async () => await toggleLanguage()}>
        {useAmharic ? "English" : "አማርኛ"}
      </button>
      <button class="control-button" onclick={async () => await toggleNumbers()}>
        {useGeezNumbers ? "1234" : "፩፪፫፬"}
      </button>
      <button class="control-button" onclick={async () => await toggleNumericFormat()}>
        {useNumericFormat ? "Text" : "DD/MM"}
      </button>
    </div>

    <div class="calendar-controls secondary-controls">
      <button
        class="control-button {showQen ? 'enabled' : 'disabled'}"
        onclick={async () => await toggleShowQen()}
      >
        ቀን
      </button>
      <button
        class="control-button {showAmeteMihret ? 'enabled' : 'disabled'}"
        onclick={async () => await toggleShowAmeteMihret()}
      >
        ዓ.ም.
      </button>
      <button
        class="control-button {showSpecialDayOptions ? 'enabled' : 'disabled'}"
        onclick={async () => await toggleSpecialDayOptions()}
      >
        {useAmharic ? "በዓላት" : "Special Days"} {showSpecialDayOptions ? "▲" : "▼"}
      </button>
    </div>

    {#if showSpecialDayOptions}
      <div class="calendar-controls tertiary-controls">
        <button
          class="control-button {showNationalSpecialDays ? 'enabled' : 'disabled'}"
          onclick={async () => await toggleNationalSpecialDays()}
        >
          {useAmharic ? "ብሔራዊ" : "National"}
        </button>
        <button
          class="control-button {showGovernmentSpecialDays ? 'enabled' : 'disabled'}"
          onclick={async () => await toggleGovernmentSpecialDays()}
        >
          {useAmharic ? "መንግስት" : "Government"}
        </button>
        <button
          class="control-button {showChristianSpecialDays ? 'enabled' : 'disabled'}"
          onclick={async () => await toggleChristianSpecialDays()}
        >
          {useAmharic ? "ክርስትና" : "Christian"}
        </button>
        <button
          class="control-button {showMuslimSpecialDays ? 'enabled' : 'disabled'}"
          onclick={async () => await toggleMuslimSpecialDays()}
        >
          {useAmharic ? "እስልምና" : "Muslim"}
        </button>
      </div>
    {/if}

    <div class="calendar-grid">
      {#each (useAmharic ? weekdaysAmharic : weekdaysEnglish) as weekday}
        <div class="weekday-header">{weekday}</div>
      {/each}

      {#each emptyStartCells as _}
        <div class="calendar-day empty"></div>
      {/each}

      {#each calendarMonth.days as day}
        <div
          class="calendar-day {day.is_today ? 'today' : ''} {dayHasVisibleSpecialDay(day) ? 'holiday' : ''}"
          title={getSpecialDayTooltip(day)}
        >
          <span class="day-number">{getDisplayNumber(day)}</span>
          {#if dayHasVisibleSpecialDay(day)}
            <span class="holiday-dot"></span>
          {/if}
        </div>
      {/each}
    </div>

    {#if monthSpecialDays.length > 0}
      <div class="special-days-list">
        <div class="special-days-title">{useAmharic ? "የበዓላት ዝርዝር" : "Special Days"}</div>
        {#each monthSpecialDays as item}
          <div class="special-day-item">
            <span class="special-day-date">
              {useGeezNumbers ? item.day.day_geez : item.day.day.toString()}
            </span>
            <span class="special-day-name">{getSpecialDayDisplayName(item.specialDay)}</span>
          </div>
        {/each}
      </div>
    {/if}

    {#if currentDate}
      <div class="current-date-info">
        <div class="today-info">
          {useAmharic ? "ዛሬ" : "Today"}: {getTodayDateDisplay()}
          <button class="copy-button" onclick={copyDateToClipboard} title="Copy date to clipboard">
            📋
          </button>
        </div>
      </div>
    {/if}
  {:else}
    <div class="loading">Loading calendar...</div>
  {/if}
</div>

<style>
  .calendar-container {
    width: 100%;
    max-width: 320px;
    margin: 0 auto;
    padding: 20px;
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(20px);
    border-radius: 16px;
    box-shadow:
      0 0 0 0.5px rgba(0, 0, 0, 0.04),
      0 2px 4px rgba(0, 0, 0, 0.05),
      0 8px 16px rgba(0, 0, 0, 0.08);
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', 'Helvetica Neue', sans-serif;
    font-feature-settings: "kern" 1, "liga" 1;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    min-height: fit-content;
    height: auto;
  }

  .calendar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 20px;
  }

  .nav-button {
    background: rgba(0, 0, 0, 0.04);
    border: none;
    font-size: 20px;
    cursor: pointer;
    padding: 8px 10px;
    border-radius: 8px;
    color: rgba(0, 0, 0, 0.7);
    transition: all 0.15s ease;
    font-weight: 500;
    display: flex;
    align-items: center;
    justify-content: center;
    min-width: 36px;
    height: 36px;
  }

  .nav-button:hover {
    background: rgba(0, 0, 0, 0.08);
    transform: scale(1.05);
  }

  .nav-button:active {
    transform: scale(0.95);
  }

  .month-year {
    text-align: center;
    flex: 1;
  }

  .month-name {
    margin: 0;
    font-size: 20px;
    font-weight: 600;
    color: rgba(0, 0, 0, 0.9);
    letter-spacing: -0.01em;
  }

  .year {
    font-size: 14px;
    color: rgba(0, 0, 0, 0.6);
    margin-top: 2px;
    font-weight: 500;
  }

  .calendar-controls {
    display: flex;
    gap: 8px;
    margin-bottom: 20px;
    justify-content: center;
    flex-wrap: wrap;
  }

  .secondary-controls {
    margin-bottom: 15px;
    margin-top: -10px;
  }

  .tertiary-controls {
    margin-bottom: 14px;
    margin-top: -8px;
  }

  .control-button {
    background: rgba(0, 0, 0, 0.04);
    border: none;
    padding: 6px 12px;
    border-radius: 8px;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
    color: rgba(0, 0, 0, 0.8);
    white-space: nowrap;
  }

  .control-button:hover {
    background: rgba(0, 0, 0, 0.08);
    transform: translateY(-1px);
  }

  .control-button:active {
    transform: translateY(0);
  }

  .control-button.enabled {
    background: rgba(0, 120, 255, 0.1);
    color: rgba(0, 120, 255, 0.9);
    border: 1px solid rgba(0, 120, 255, 0.2);
  }

  .control-button.enabled:hover {
    background: rgba(0, 120, 255, 0.15);
    transform: translateY(-1px);
  }

  .control-button.disabled {
    background: rgba(0, 0, 0, 0.02);
    color: rgba(0, 0, 0, 0.4);
    border: 1px solid rgba(0, 0, 0, 0.1);
  }

  .control-button.disabled:hover {
    background: rgba(0, 0, 0, 0.04);
    transform: translateY(-1px);
  }

  .calendar-grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 2px;
    background: transparent;
    border-radius: 12px;
    overflow: hidden;
  }

  .weekday-header {
    background: transparent;
    padding: 12px 4px 8px;
    text-align: center;
    font-size: 11px;
    font-weight: 600;
    color: rgba(0, 0, 0, 0.5);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .calendar-day {
    background: transparent;
    min-height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    border-radius: 8px;
    transition: all 0.15s ease;
    cursor: default;
  }

  .calendar-day:hover:not(.empty) {
    background: rgba(0, 0, 0, 0.04);
  }

  .calendar-day.empty {
    background: transparent;
    opacity: 0;
  }

  .calendar-day.today {
    background: #007AFF;
    color: white;
    font-weight: 600;
    box-shadow: 0 2px 8px rgba(0, 122, 255, 0.3);
  }

  .calendar-day.today:hover {
    background: #0056CC;
    transform: scale(1.05);
  }

  .calendar-day.holiday:not(.today) {
    box-shadow: inset 0 0 0 1px rgba(255, 149, 0, 0.35);
  }

  .day-number {
    font-size: 15px;
    font-weight: 500;
    line-height: 1;
  }

  .holiday-dot {
    position: absolute;
    bottom: 5px;
    width: 5px;
    height: 5px;
    border-radius: 999px;
    background: #FF9500;
  }

  .calendar-day.today .holiday-dot {
    background: rgba(255, 255, 255, 0.95);
  }

  .special-days-list {
    margin-top: 14px;
    padding: 10px 12px;
    border-radius: 10px;
    border: 1px solid rgba(0, 0, 0, 0.08);
    background: rgba(0, 0, 0, 0.02);
  }

  .special-days-title {
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.3px;
    text-transform: uppercase;
    color: rgba(0, 0, 0, 0.6);
    margin-bottom: 8px;
  }

  .special-day-item {
    display: flex;
    align-items: baseline;
    gap: 8px;
    font-size: 12px;
    padding: 2px 0;
  }

  .special-day-date {
    min-width: 20px;
    font-weight: 600;
    color: rgba(0, 0, 0, 0.75);
  }

  .special-day-name {
    color: rgba(0, 0, 0, 0.65);
  }

  .current-date-info {
    margin-top: 20px;
    padding-top: 16px;
    border-top: 1px solid rgba(0, 0, 0, 0.08);
  }

  .today-info {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    text-align: center;
    font-size: 13px;
    color: rgba(0, 0, 0, 0.6);
    font-weight: 500;
  }

  .copy-button {
    background: none;
    border: none;
    font-size: 14px;
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    opacity: 0.6;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .copy-button:hover {
    opacity: 1;
    background: rgba(0, 0, 0, 0.05);
    transform: scale(1.1);
  }

  .loading {
    text-align: center;
    padding: 40px;
    color: rgba(0, 0, 0, 0.6);
    font-weight: 500;
  }

  @media (prefers-color-scheme: dark) {
    .calendar-container {
      background: rgba(30, 30, 30, 0.95);
      backdrop-filter: blur(20px);
      box-shadow:
        0 0 0 0.5px rgba(255, 255, 255, 0.08),
        0 2px 4px rgba(0, 0, 0, 0.3),
        0 8px 16px rgba(0, 0, 0, 0.4);
    }

    .month-name {
      color: rgba(255, 255, 255, 0.95);
    }

    .year {
      color: rgba(255, 255, 255, 0.6);
    }

    .nav-button {
      background: rgba(255, 255, 255, 0.08);
      color: rgba(255, 255, 255, 0.8);
    }

    .nav-button:hover {
      background: rgba(255, 255, 255, 0.15);
    }

    .control-button {
      background: rgba(255, 255, 255, 0.08);
      color: rgba(255, 255, 255, 0.9);
    }

    .control-button:hover {
      background: rgba(255, 255, 255, 0.15);
    }

    .control-button.enabled {
      background: rgba(100, 200, 255, 0.15);
      color: rgba(100, 200, 255, 0.95);
      border: 1px solid rgba(100, 200, 255, 0.3);
    }

    .control-button.enabled:hover {
      background: rgba(100, 200, 255, 0.25);
    }

    .control-button.disabled {
      background: rgba(255, 255, 255, 0.03);
      color: rgba(255, 255, 255, 0.4);
      border: 1px solid rgba(255, 255, 255, 0.1);
    }

    .control-button.disabled:hover {
      background: rgba(255, 255, 255, 0.06);
    }

    .weekday-header {
      color: rgba(255, 255, 255, 0.5);
    }

    .calendar-day {
      color: rgba(255, 255, 255, 0.9);
    }

    .calendar-day:hover:not(.empty) {
      background: rgba(255, 255, 255, 0.08);
    }

    .calendar-day.today {
      background: #0A84FF;
      box-shadow: 0 2px 8px rgba(10, 132, 255, 0.4);
    }

    .calendar-day.today:hover {
      background: #0066CC;
    }

    .calendar-day.holiday:not(.today) {
      box-shadow: inset 0 0 0 1px rgba(255, 174, 66, 0.45);
    }

    .holiday-dot {
      background: #FFB55A;
    }

    .special-days-list {
      border-color: rgba(255, 255, 255, 0.15);
      background: rgba(255, 255, 255, 0.04);
    }

    .special-days-title {
      color: rgba(255, 255, 255, 0.7);
    }

    .special-day-date {
      color: rgba(255, 255, 255, 0.88);
    }

    .special-day-name {
      color: rgba(255, 255, 255, 0.72);
    }

    .current-date-info {
      border-top-color: rgba(255, 255, 255, 0.12);
    }

    .today-info {
      color: rgba(255, 255, 255, 0.6);
    }

    .copy-button:hover {
      background: rgba(255, 255, 255, 0.08);
    }

    .loading {
      color: rgba(255, 255, 255, 0.6);
    }
  }
</style>
