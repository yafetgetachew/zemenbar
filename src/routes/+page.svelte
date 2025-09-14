<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface AppSettings {
    use_amharic: boolean;
    use_geez_numbers: boolean;
  }

  let settings: AppSettings = { use_amharic: true, use_geez_numbers: false };

  async function loadSettings() {
    try { settings = await invoke("load_settings"); } catch (e) { console.error(e); }
  }
  async function saveSettings() {
    try {
      await invoke("save_settings", { settings });
      await invoke("update_tray_display");
    } catch (e) { console.error(e); }
  }
  async function setLanguage(amharic: boolean) { settings.use_amharic = amharic; await saveSettings(); }
  async function setNumerals(geez: boolean) { settings.use_geez_numbers = geez; await saveSettings(); }

  onMount(async () => { await loadSettings(); });
</script>

<main class="container">
  <div class="panel">
    <div class="row">
      <div class="label">Language</div>
      <div class="options">
        <label><input type="radio" name="lang" checked={!settings.use_amharic} on:change={() => setLanguage(false)} /> English</label>
        <label><input type="radio" name="lang" checked={settings.use_amharic} on:change={() => setLanguage(true)} /> አማርኛ</label>
      </div>
    </div>
    <div class="row">
      <div class="label">Numerals</div>
      <div class="options">
        <label><input type="radio" name="nums" checked={!settings.use_geez_numbers} on:change={() => setNumerals(false)} /> Arabic</label>
        <label><input type="radio" name="nums" checked={settings.use_geez_numbers} on:change={() => setNumerals(true)} /> ግዕዝ</label>
      </div>
    </div>
  </div>
</main>

<style>
  :global(html, body) { margin: 0; padding: 0; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: transparent; }
  :root { color-scheme: light dark; }
  .container { width: 100vw; height: 100vh; display: flex; align-items: flex-start; justify-content: center; background: transparent; }
  .panel { background: var(--panel-bg); border-radius: 10px; padding: 6px 12px 10px; width: 320px; box-shadow: 0 8px 26px rgba(0,0,0,0.14); border: 1px solid var(--panel-border); backdrop-filter: saturate(180%) blur(12px); }
  .row { display: grid; grid-template-columns: 100px 1fr; align-items: center; gap: 10px; padding: 10px 0; }
  .row + .row { border-top: 1px solid var(--divider); }
  .label { color: var(--label); font-size: 13px; }
  .options { display: flex; gap: 12px; align-items: center; justify-content: flex-end; flex-wrap: wrap; }
  label { color: var(--text); font-size: 13px; display: inline-flex; align-items: center; gap: 8px; cursor: pointer; }

  .options input[type="radio"] {
    appearance: none;
    -webkit-appearance: none;
    width: 16px;
    height: 16px;
    border: 1.5px solid var(--panel-border);
    border-radius: 4px;
    background: transparent;
    display: inline-block;
    position: relative;
  }
  .options input[type="radio"]:checked {
    background: #22c55e; /* green */
    border-color: #22c55e;
  }
  .options input[type="radio"]:focus-visible {
    outline: 2px solid #22c55e66;
    outline-offset: 2px;
  }

  /* Light theme */
  :root { --panel-bg: rgba(255,255,255,0.92); --panel-border: rgba(0,0,0,0.18); --divider: rgba(0,0,0,0.06); --label: #555; --text: #222; }
  @media (prefers-color-scheme: dark) {
    :root { --panel-bg: rgba(22,22,24,0.86); --panel-border: rgba(255,255,255,0.24); --divider: rgba(255,255,255,0.12); --label: #c9c9c9; --text: #f3f3f3; }
  }
</style>
