// Не показывать консольное окно в релизной сборке на Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use tauri::{webview::NewWindowResponse, WebviewUrl, WebviewWindowBuilder};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            let counter = Arc::new(AtomicUsize::new(0));

            // Главное окно создаём вручную, чтобы перехватить window.open().
            // Программа открывает бланк рецепта через window.open('about:blank')
            // и пишет в него документ (document.write) — поэтому нужно настоящее
            // дочернее окно, а не отказ.
            WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("Выписка рецептов для лечения ССЗ")
                .inner_size(1280.0, 860.0)
                .min_inner_size(900.0, 600.0)
                .center()
                .on_new_window(move |_url, features| {
                    let n = counter.fetch_add(1, Ordering::Relaxed);
                    let built = WebviewWindowBuilder::new(
                        &handle,
                        format!("rp-{n}"),
                        WebviewUrl::External("about:blank".parse().unwrap()),
                    )
                    .title("Рецепт — форма 107-1/у")
                    .inner_size(900.0, 1000.0)
                    .window_features(features)
                    .on_document_title_changed(|window, title| {
                        let _ = window.set_title(&title);
                    })
                    .build();

                    match built {
                        Ok(window) => NewWindowResponse::Create { window },
                        Err(_) => NewWindowResponse::Deny,
                    }
                })
                .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("ошибка запуска приложения");
}
