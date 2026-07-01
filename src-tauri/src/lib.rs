use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WindowEvent,
};

fn is_chinese_locale() -> bool {
    sys_locale::get_locale()
        .map(|l| l.starts_with("zh"))
        .unwrap_or(false)
}

fn tray_labels() -> (&'static str, &'static str, &'static str, &'static str) {
    if is_chinese_locale() {
        ("显示", "隐藏", "退出", "DeepSeek")
    } else {
        ("Show", "Hide", "Quit", "DeepSeek")
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let (show_text, hide_text, quit_text, tooltip) = tray_labels();

            // Build tray menu
            let show = MenuItemBuilder::with_id("show", show_text).build(app)?;
            let hide = MenuItemBuilder::with_id("hide", hide_text).build(app)?;
            let quit = MenuItemBuilder::with_id("quit", quit_text).build(app)?;

            let menu = MenuBuilder::new(app)
                .item(&show)
                .item(&hide)
                .separator()
                .item(&quit)
                .build()?;

            // Build tray icon
            let icon =
                Image::from_bytes(include_bytes!("../icons/32x32.png")).expect("load tray icon");

            TrayIconBuilder::new()
                .icon(icon)
                .tooltip(tooltip)
                .menu(&menu)
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "hide" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.hide();
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().ok().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            // Create the main window
            let window = tauri::webview::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::App("index.html".into()),
            )
            .title("DeepSeek")
            .inner_size(1200.0, 800.0)
            .min_inner_size(800.0, 600.0)
            .on_document_title_changed(|window, title| {
                let _ = window.set_title(&title);
            })
            .build()?;

            // Minimize to tray instead of closing
            let w = window.clone();
            window.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = w.hide();
                }
            });

            window.set_title("DeepSeek").ok();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
