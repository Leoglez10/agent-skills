//! Native menu bar. Menu items carry the frontend command id (src/lib/commands.svelte.ts); choosing one
//! emits `menu` with that id to the main window, which runs it. Accelerators live here only.
//! Replace APP with the product name.
use tauri::{AppHandle, Emitter, Manager, menu::{AboutMetadata, Menu, MenuItem, PredefinedMenuItem, Submenu}};

fn item(app: &AppHandle, id: &str, label: &str, accel: Option<&str>) -> tauri::Result<MenuItem<tauri::Wry>> {
    MenuItem::with_id(app, id, label, true, accel)
}

pub fn build(app: &AppHandle) -> tauri::Result<Menu<tauri::Wry>> {
    let sep = || PredefinedMenuItem::separator(app);
    let main = Submenu::with_items(app, "APP", true, &[
        &PredefinedMenuItem::about(app, Some("Acerca de APP"), Some(AboutMetadata::default()))?, &sep()?,
        &item(app, "go-settings", "Ajustes…", Some("CmdOrCtrl+,"))?, &sep()?,
        &PredefinedMenuItem::hide(app, Some("Ocultar APP"))?, &PredefinedMenuItem::hide_others(app, Some("Ocultar otros"))?, &sep()?,
        &PredefinedMenuItem::quit(app, Some("Salir de APP"))?,
    ])?;
    let file = Submenu::with_items(app, "Archivo", true, &[
        &item(app, "new-item", "Nuevo <elemento>", Some("CmdOrCtrl+N"))?,
        &sep()?, &PredefinedMenuItem::close_window(app, Some("Cerrar ventana"))?,
    ])?;
    // Undo/redo/cut/copy/paste stay native so text fields behave; the app's ⌘Z runs in the webview
    // first and only when no field is being edited.
    let edit = Submenu::with_items(app, "Edición", true, &[
        &PredefinedMenuItem::undo(app, Some("Deshacer"))?, &PredefinedMenuItem::redo(app, Some("Rehacer"))?, &sep()?,
        &PredefinedMenuItem::cut(app, Some("Cortar"))?, &PredefinedMenuItem::copy(app, Some("Copiar"))?, &PredefinedMenuItem::paste(app, Some("Pegar"))?,
        &PredefinedMenuItem::select_all(app, Some("Seleccionar todo"))?, &sep()?,
        &item(app, "search", "Buscar <elementos>", Some("CmdOrCtrl+F"))?,
    ])?;
    let view = Submenu::with_items(app, "Ver", true, &[
        &item(app, "go-home", "Inicio", Some("CmdOrCtrl+1"))?,
        &item(app, "go-list", "<pantalla 2>", Some("CmdOrCtrl+2"))?, &sep()?,
        &item(app, "sidebar", "Mostrar u ocultar la barra lateral", Some("CmdOrCtrl+S"))?,
        &item(app, "theme", "Cambiar entre claro y oscuro", Some("CmdOrCtrl+Shift+L"))?,
        &PredefinedMenuItem::fullscreen(app, Some("Pantalla completa"))?,
    ])?;
    let window = Submenu::with_items(app, "Ventana", true, &[
        &item(app, "palette", "Paleta de comandos", Some("CmdOrCtrl+K"))?, &sep()?,
        &PredefinedMenuItem::minimize(app, Some("Minimizar"))?, &PredefinedMenuItem::maximize(app, Some("Zoom"))?,
    ])?;
    Menu::with_items(app, &[&main, &file, &edit, &view, &window])
}

/// Sends a command id to the main window (shown first, since menus also work while it is hidden).
pub fn dispatch(app: &AppHandle, id: &str) {
    if let Some(w) = app.get_webview_window("main") { let _ = w.show(); let _ = w.set_focus(); }
    if let Err(e) = app.emit_to("main", "menu", id) { eprintln!("Menú: {e}"); }
}
