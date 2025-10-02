use tokio::runtime::*;

mod app;
mod audio;
mod components;
mod prelude;
mod server;

use crate::app::*;
use crate::server::*;

fn audio_service(rt: Runtime) {
    rt.block_on(run()).unwrap();
}

fn main() {
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    std::thread::spawn(|| audio_service(rt));

    dioxus::launch(App);
}
