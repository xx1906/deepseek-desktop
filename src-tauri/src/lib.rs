#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = tauri::webview::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::App("index.html".into()),
            )
            .title("DeepSeek")
            .inner_size(1200.0, 800.0)
            .min_inner_size(800.0, 600.0)
            .initialization_script(
                r#"(function(){var T="DeepSeek";try{Object.defineProperty(document,"title",{configurable:false,enumerable:true,get:function(){return T},set:function(){}})}catch(e){document.title=T}})();"#,
            )
            .on_document_title_changed(|window, title| {
                if title != "DeepSeek" {
                    let _ = window.set_title("DeepSeek");
                }
            })
            .build()?;

            window.set_title("DeepSeek").ok();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
