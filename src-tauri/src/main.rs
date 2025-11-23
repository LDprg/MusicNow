// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // fix linux wayland NVIDIA problems until fixed upstream
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

    musicnow_lib::run()
}
