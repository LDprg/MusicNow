use musicnow_shared::add;
use regex::Regex;
use scraper::{self, Html, Selector};
use tauri::async_runtime::block_on;

async fn get_client_id() -> String {
    let resp = reqwest::get("https://soundcloud.com").await.unwrap();
    let body = resp.text().await.unwrap();

    let site = Html::parse_document(&body);
    let script = Selector::parse("script[src]").unwrap();
    let script_regex = Regex::new(r"https://.*/assets/0.*.js").unwrap();

    let mut script_src = "";

    for element in site.select(&script) {
        if let Some(src) = element.value().attr("src")
            && script_regex.is_match(src)
        {
            script_src = src;
        }
    }

    println!("{}", script_src);

    let resp = reqwest::get(script_src).await.unwrap();
    let body = resp.text().await.unwrap();

    let start = "\"client_id=";
    let end = "\"";

    let pos = body.find(start).unwrap();
    let client_id = body.get(pos + start.len()..).unwrap();

    let pos = client_id.find(end).unwrap();
    let client_id = client_id.get(..pos).unwrap();

    println!("{}", client_id);

    client_id.to_string()
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    let client_id = block_on(get_client_id());

    format!(
        "Hello, {}! You've been greeted from Rust! With client_id: \"{}\"",
        name, client_id
    )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    console_error_panic_hook::set_once();

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .setup(|_| {
            log::warn!("{}", add(1, 2));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
