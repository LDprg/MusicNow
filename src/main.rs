#[cfg(feature = "server")]
use std::thread;

use crate::prelude::*;

mod app;
mod components;
mod prelude;
mod server;

use crate::app::*;

fn main() {
    dioxus::logger::init(Level::INFO).expect("failed to init logger");

    #[cfg(feature = "server")]
    thread::spawn(service::spawn);

    #[cfg(not(feature = "server"))]
    dioxus::fullstack::set_server_url("http://127.0.0.1:8080");

    dioxus::LaunchBuilder::new()
        .with_cfg(desktop!(dioxus::desktop::Config::new().with_menu(None)))
        .launch(App);
}
