use crate::prelude::*;

mod app;
mod components;
mod prelude;
mod services;

use crate::app::*;

fn main() {
    dioxus::logger::init(Level::INFO).expect("failed to init logger");

    services::spawn();

    dioxus::LaunchBuilder::new()
        .with_cfg(desktop!(dioxus::desktop::Config::new().with_menu(None)))
        .launch(App);
}
