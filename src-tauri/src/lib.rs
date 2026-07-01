use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WindowEvent,
};

struct TrayLabels {
    show: &'static str,
    hide: &'static str,
    quit: &'static str,
    tooltip: &'static str,
}

fn tray_labels() -> TrayLabels {
    match sys_locale::get_locale().as_deref() {
        Some(l) if l.starts_with("zh") => TrayLabels {
            show: "显示",
            hide: "隐藏",
            quit: "退出",
            tooltip: "DeepSeek",
        },
        Some(l) if l.starts_with("ja") => TrayLabels {
            show: "表示",
            hide: "非表示",
            quit: "終了",
            tooltip: "DeepSeek",
        },
        Some(l) if l.starts_with("ko") => TrayLabels {
            show: "표시",
            hide: "숨기기",
            quit: "종료",
            tooltip: "DeepSeek",
        },
        Some(l) if l.starts_with("fr") => TrayLabels {
            show: "Afficher",
            hide: "Masquer",
            quit: "Quitter",
            tooltip: "DeepSeek",
        },
        Some(l) if l.starts_with("de") => TrayLabels {
            show: "Anzeigen",
            hide: "Ausblenden",
            quit: "Beenden",
            tooltip: "DeepSeek",
        },
        Some(l) if l.starts_with("es") => TrayLabels {
            show: "Mostrar",
            hide: "Ocultar",
            quit: "Salir",
            tooltip: "DeepSeek",
        },
        Some(l) if l.starts_with("pt") => TrayLabels {
            show: "Mostrar",
            hide: "Ocultar",
            quit: "Sair",
            tooltip: "DeepSeek",
        },
        Some(l) if l.starts_with("ru") => TrayLabels {
            show: "Показать",
            hide: "Скрыть",
            quit: "Выйти",
            tooltip: "DeepSeek",
        },
        Some(l) if l.starts_with("it") => TrayLabels {
            show: "Mostra",
            hide: "Nascondi",
            quit: "Esci",
            tooltip: "DeepSeek",
        },
        Some(l) if l.starts_with("nl") => TrayLabels {
            show: "Tonen",
            hide: "Verbergen",
            quit: "Afsluiten",
            tooltip: "DeepSeek",
        },
        Some(l) if l.starts_with("pl") => TrayLabels {
            show: "Pokaż",
            hide: "Ukryj",
            quit: "Wyjdź",
            tooltip: "DeepSeek",
        },
        Some(l) if l.starts_with("tr") => TrayLabels {
            show: "Göster",
            hide: "Gizle",
            quit: "Çıkış",
            tooltip: "DeepSeek",
        },
        Some(l) if l.starts_with("vi") => TrayLabels {
            show: "Hiện",
            hide: "Ẩn",
            quit: "Thoát",
            tooltip: "DeepSeek",
        },
        Some(l) if l.starts_with("th") => TrayLabels {
            show: "แสดง",
            hide: "ซ่อน",
            quit: "ออก",
            tooltip: "DeepSeek",
        },
        Some(l) if l.starts_with("ar") => TrayLabels {
            show: "إظهار",
            hide: "إخفاء",
            quit: "خروج",
            tooltip: "DeepSeek",
        },
        Some(l) if l.starts_with("hi") => TrayLabels {
            show: "दिखाएँ",
            hide: "छिपाएँ",
            quit: "बाहर निकलें",
            tooltip: "DeepSeek",
        },
        _ => TrayLabels {
            show: "Show",
            hide: "Hide",
            quit: "Quit",
            tooltip: "DeepSeek",
        },
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .setup(|app| {
            let labels = tray_labels();

            // Build tray menu
            let show = MenuItemBuilder::with_id("show", labels.show).build(app)?;
            let hide = MenuItemBuilder::with_id("hide", labels.hide).build(app)?;
            let quit = MenuItemBuilder::with_id("quit", labels.quit).build(app)?;

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
                .tooltip(labels.tooltip)
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
