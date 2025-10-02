use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::hi_solid_icons::{HiArrowCircleLeft, HiArrowCircleRight, HiPause, HiPlay},
};

mod components;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx!(
        div { "Site Something" }
        div { class: "absolute inset-x-0 bottom-0 m-[8]",
            progress { class: "progress w-full", value: 30, max: 100 }
            div { class: "flex justify-items-center items-center w-full",
                div { class: "flex-1 mx-8 justify-self-start align-center", "Song" }
                button { class: "btn btn-square", width: 50, height: 50,
                    Icon { width: 50, height: 50, icon: HiArrowCircleLeft }
                }
                button { class: "btn btn-square", width: 50, height: 50,
                    Icon { width: 50, height: 50, icon: HiPlay }
                }
                button { class: "btn btn-square", width: 50, height: 50,
                    Icon { width: 50, height: 50, icon: HiPause }
                }
                button { class: "btn btn-square", width: 50, height: 50,
                    Icon { width: 50, height: 50, icon: HiArrowCircleRight }
                }
                div { class: "flex-1 ml-[50]",
                    "Volume: "
                    input {
                        class: "px-[8] w-9/10 range range-xs",
                        r#type: "range",
                        min: 0,
                        max: 100,
                        value: 40,
                    }
                }
            }
        }
    )
}
