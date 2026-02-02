use tauri::Manager;

const OPENPLATFORM_URL: &str = "http://192.168.195.33:3100";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .setup(|app| {
            // Get the main window and navigate to OpenPlatform
            if let Some(window) = app.get_webview_window("main") {
                // Navigate to OpenPlatform URL
                let _ = window.eval(&format!(
                    "window.location.replace('{}')",
                    OPENPLATFORM_URL
                ));
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
