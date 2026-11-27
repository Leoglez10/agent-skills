// <Topic>: one line saying what behavior this proves.
// Run: qa/run.sh qa/<topic>.js   (prints PASS/FAIL lines; exit 1 on any FAIL)
(async () => {
  const wait = ms => window.__TAURI_INTERNALS__.invoke('qa_sleep', { ms });
  const say = (ok, msg) => window.__TAURI_INTERNALS__.invoke('qa_log', { line: (ok ? 'PASS ' : 'FAIL ') + msg });
  const $ = s => document.querySelector(s);
  // Retries for up to 4s, so checks do not race renders and transitions.
  const ok = async (check, message) => { let pass = false; for (let i = 0; i < 40 && !(pass = !!(await check())); i++) await wait(100); say(pass, message()); };
  try {
    await wait(1500); // let the app mount before reading window.__app or the DOM
    // Example: the sidebar toggle collapses it and the choice is saved.
    const before = $('.sidebar').getBoundingClientRect().width;
    $('.side-toggle').click();
    await wait(500);
    await ok(() => $('.sidebar.collapsed'), () => `the toggle collapses the sidebar (${before}px → ${Math.round($('.sidebar').getBoundingClientRect().width)}px)`);
    await ok(() => localStorage.getItem('sidebar-collapsed') === '1', () => 'the choice is saved');
    // Leave the app as you found it, so the next script starts clean.
    $('.side-toggle').click(); await wait(500);
  } catch (e) { say(false, 'exception: ' + e); }
  window.__TAURI_INTERNALS__.invoke('qa_log', { line: 'DONE' });
})();
