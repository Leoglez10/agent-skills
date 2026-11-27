//! App entry, taken from Finanzas (desktop/src-tauri/src/lib.rs). Replace APP_DEV_EVAL and add your commands.
//! Cargo deps: tauri 2 (features: tray-icon), tauri-plugin-{autostart,opener,global-shortcut,updater,process,dialog,notification}.
mod menu;
mod tray;

use std::time::Duration;

#[tauri::command]
fn qa_log(line: String) {
    // Debug builds only: QA scripts report PASS/FAIL lines to stderr; release builds ignore them.
    #[cfg(debug_assertions)]
    eprintln!("QA {line}");
    #[cfg(not(debug_assertions))]
    let _ = line;
}

#[tauri::command(async)]
fn qa_sleep(ms: u64) {
    #[cfg(debug_assertions)]
    std::thread::sleep(Duration::from_millis(ms.min(10_000)));
    #[cfg(not(debug_assertions))]
    let _ = ms;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, None))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            // The windows come from tauri.conf.json ("create": false) so QA runs can keep a covered window alive.
            let config = app.config().app.windows.first().cloned().ok_or("missing main window in tauri.conf.json")?;
            #[allow(unused_mut)]
            let mut window = tauri::WebviewWindowBuilder::from_config(app.handle(), &config)?;
            #[cfg(debug_assertions)]
            if std::env::var_os("APP_DEV_EVAL").is_some() {
                window = window.background_throttling(tauri::utils::config::BackgroundThrottlingPolicy::Disabled);
            }
            window.build()?;
            if let Some(quick) = app.config().app.windows.iter().find(|w| w.label == "quick").cloned() {
                tauri::WebviewWindowBuilder::from_config(app.handle(), &quick)?.build()?;
            }
            app.set_menu(menu::build(app.handle())?)?;
            app.on_menu_event(|app, event| menu::dispatch(app, event.id().as_ref()));
            // menu::register_shortcut(app.handle());   // global quick-capture shortcut
            tray::refresh(app.handle());
            Ok(())
        })
        // With the tray icon on, closing the window only hides it so reminders keep arriving.
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } if window.label() == "main" && window.app_handle().tray_by_id("app").is_some() => {
                api.prevent_close();
                let _ = window.hide();
            }
            // Quick capture behaves like Spotlight: clicking elsewhere puts it away.
            tauri::WindowEvent::Focused(false) if window.label() == "quick" => { let _ = window.hide(); }
            _ => {}
        })
        // Debug builds only: APP_DEV_EVAL runs a script after the page loads, to drive the real UI in checks.
        .on_page_load(|_webview, _payload| {
            #[cfg(debug_assertions)]
            if _payload.event() == tauri::webview::PageLoadEvent::Finished && _webview.label() == "main" {
                if let Ok(js) = std::env::var("APP_DEV_EVAL") {
                    if let Err(e) = _webview.eval(&js) { eprintln!("APP_DEV_EVAL: {e}"); }
                }
            }
        })
        .invoke_handler(tauri::generate_handler![qa_log, qa_sleep /* , your commands, tray::refresh_tray */])
        .run(tauri::generate_context!())
        .expect("error while starting the app");
}
