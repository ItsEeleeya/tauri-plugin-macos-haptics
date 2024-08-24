use tauri::{Manager, WindowEvent};
use tauri_plugin_decorum::WebviewWindowExt;

const TRAFFIC_LIGHT_INSET_X: f32 = 15.0;
const TRAFFIC_LIGHT_INSET_Y: f32 = 18.0;

pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(target_os = "macos")]
    {
        builder = builder.plugin(tauri_plugin_macos_haptics::init());
    }

    builder
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            if let Some(window) = app.get_webview_window("main") {
                #[cfg(target_os = "macos")]
                {
                    window.make_transparent().ok();
                    window
                        .set_traffic_lights_inset(TRAFFIC_LIGHT_INSET_X, TRAFFIC_LIGHT_INSET_Y)
                        .ok();
                }
            }
            Ok(())
        })
        .on_window_event(move |window, event| match event {
            WindowEvent::ThemeChanged(_theme) => {
                #[cfg(target_os = "macos")]
                if let Some(webview) = window.get_webview_window("main") {
                    webview
                        .set_traffic_lights_inset(TRAFFIC_LIGHT_INSET_X, TRAFFIC_LIGHT_INSET_Y)
                        .ok();
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
