//! Menu-bar icon: a live value as its title, the next pending item, and a way back to the window.
//! While it exists, closing the window only hides it (see lib.rs). Errors are logged, never fatal.
use tauri::{AppHandle, Manager, menu::{Menu, MenuItem, PredefinedMenuItem}, tray::TrayIconBuilder};

const ID: &str = "app";

/// Shows the main window, optionally on a route (`proximos`, `inicio`…).
pub fn show(app: &AppHandle, route: Option<&str>) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
        if let Some(route) = route { let _ = window.eval(format!("location.hash = {route:?}")); }
    }
}

/// Replace with a read from your core crate: (title, next pending item and whether it is overdue).
fn summary(_app: &AppHandle) -> Result<(String, Option<(String, bool)>), String> {
    Ok(("—".into(), None))
}

fn menu(app: &AppHandle, title: &str, next: &Option<(String, bool)>) -> tauri::Result<Menu<tauri::Wry>> {
    let value = MenuItem::with_id(app, "value", title, false, None::<&str>)?;
    let next = match next {
        Some((text, overdue)) => MenuItem::with_id(app, "next", if *overdue { format!("⚠︎ {text}") } else { format!("Próximo: {text}") }, true, None::<&str>)?,
        None => MenuItem::with_id(app, "next", "Nada pendiente", false, None::<&str>)?,
    };
    let quick = MenuItem::with_id(app, "quick", "Captura rápida", true, Some("Alt+Shift+F"))?;
    let open = MenuItem::with_id(app, "open", "Abrir", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Salir", true, Some("CmdOrCtrl+Q"))?;
    Menu::with_items(app, &[&value, &next, &PredefinedMenuItem::separator(app)?, &quick, &open, &quit])
}

/// Creates the icon, or updates it when it already exists. Call `remove_tray_by_id(ID)` when the user turns it off.
pub fn refresh(app: &AppHandle) {
    let (title, next) = match summary(app) { Ok(s) => s, Err(e) => { eprintln!("Tray: {e}"); return; } };
    let result = (|| -> tauri::Result<()> {
        let menu = menu(app, &title, &next)?;
        if let Some(tray) = app.tray_by_id(ID) {
            tray.set_menu(Some(menu))?;
            tray.set_title(Some(&title))?;
        } else {
            let mut builder = TrayIconBuilder::with_id(ID).menu(&menu).title(&title).tooltip("APP")
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "open" => show(app, None),
                    "next" => show(app, Some("proximos")),
                    "quick" => if let Some(w) = app.get_webview_window("quick") { let _ = w.center(); let _ = w.show(); let _ = w.set_focus(); },
                    "quit" => app.exit(0),
                    _ => {}
                });
            if let Some(icon) = app.default_window_icon() { builder = builder.icon(icon.clone()); }
            builder.build(app)?;
        }
        Ok(())
    })();
    if let Err(e) = result { eprintln!("Tray: {e}"); }
}

/// The UI calls this after it changes data so the icon never shows a stale value.
#[tauri::command(async)]
pub fn refresh_tray(app: AppHandle) { refresh(&app); }
