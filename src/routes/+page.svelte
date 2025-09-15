<script lang="ts">
  import EthiopianCalendar from "$lib/EthiopianCalendar.svelte";
</script>

<main class="container">
  <EthiopianCalendar />
</main>

<style>
  :global(html, body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    background-color: transparent;
    overflow: hidden;
  }

  .container {
    width: 100vw;
    min-height: 100vh;
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding: 0;
    margin: 0;
    background-color: transparent;
    box-sizing: border-box;
  }
</style>
