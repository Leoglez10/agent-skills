// Copy buttons: each [data-copy] button copies its attribute's text exactly, swaps its glyph to a check
// for 1.6 s and announces the result in #copy-status.
(function () {
  var status = document.getElementById('copy-status');
  function announce(msg) { if (status) status.textContent = msg; }
  function fallback(text) {
    var area = document.createElement('textarea');
    area.value = text;
    area.setAttribute('readonly', '');
    area.style.position = 'fixed';
    area.style.top = '-1000px';
    document.body.appendChild(area);
    area.select();
    var ok = false;
    try { ok = document.execCommand('copy'); } catch (e) { ok = false; }
    document.body.removeChild(area);
    return ok;
  }
  function copy(text) {
    if (navigator.clipboard && window.isSecureContext) {
      return navigator.clipboard.writeText(text).then(function () { return true; }, function () { return fallback(text); });
    }
    return Promise.resolve(fallback(text));
  }
  document.addEventListener('click', function (event) {
    var button = event.target.closest('button[data-copy]');
    if (!button) return;
    copy(button.getAttribute('data-copy') || '').then(function (ok) {
      button.setAttribute('data-copied', ok ? 'true' : 'false');
      announce(ok ? 'Copiado al portapapeles.' : 'No se pudo copiar. Selecciona el texto manualmente.');
      clearTimeout(button._copyTimer);
      button._copyTimer = setTimeout(function () { button.removeAttribute('data-copied'); announce(''); }, 1600);
    });
  });
})();
