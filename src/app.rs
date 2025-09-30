use leptos::prelude::*;
use musicnow_shared::add;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let values = vec![0, 1, 2];
    let test = add(1, 2);

    log::error!("Test {}", test);

    view! {
        <main class="container">
            {test} // this will just render "012"
            <p>{values.clone()}</p> // or we can wrap them in <li>
            <ul>{values.into_iter().map(|n| view! { <li>{n}</li> }).collect::<Vec<_>>()}</ul>
            <button on:click=move |_| {log::error!("Test123");}>Debug</button>
        </main>
    }
}
