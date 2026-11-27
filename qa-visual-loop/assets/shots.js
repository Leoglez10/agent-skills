// Visual review: walks every screen in light and dark; qa/shots.sh captures the window on each SHOT line.
(async () => {
  const wait = ms => window.__TAURI_INTERNALS__.invoke('qa_sleep', { ms });
  const log = line => window.__TAURI_INTERNALS__.invoke('qa_log', { line });
  const screens = ['inicio', 'lista', 'ajustes']; // every route of the app
  try {
    await wait(1500);
    // Optional modules are off by default: switch them on here so their screens are captured too.
    for (const theme of ['light', 'dark']) {
      document.documentElement.dataset.theme = theme;
      for (const screen of screens) {
        location.hash = screen;
        await wait(1200);
        scrollTo(0, 0);
        await log(`SHOT ${screen}-${theme}`);
        await wait(1500); // the shell captures meanwhile
      }
    }
  } catch (e) { await log('FAIL shots: ' + e); }
  await log('DONE');
})();
