use crate::prelude::*;

mod app;
mod components;
mod prelude;
mod services;

use crate::app::*;

fn main() {
    #[cfg(target_os = "linux")]
    {
        if std::path::Path::new("/dev/dri").exists()
            && std::env::var("XDG_SESSION_TYPE").unwrap_or_default() == "wayland"
        {
            // Gnome Webkit is currently buggy under Wayland and KDE, so we will run it with XWayland mode.
            // See: https://github.com/DioxusLabs/dioxus/issues/3667
            unsafe {
                // Disable explicit sync for NVIDIA drivers on Linux when using Way
                std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
            }
        }
    }

    dioxus::logger::init(Level::INFO).expect("failed to init logger");

    services::spawn();

    dioxus::LaunchBuilder::new()
        .with_cfg(desktop!(dioxus::desktop::Config::new().with_menu(None)))
        .launch(App);
}
