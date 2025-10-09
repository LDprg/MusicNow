use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::hi_solid_icons::*};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx!(
        label { class: "input w-full mb-[8]",
            Icon { class: "h-[1em]", icon: HiSearch }
            input { class: "grow", r#type: "search", placeholder: "Search" }
        }

        div { "Site Something" }
        div { "Site Something" }

        div { class: "absolute inset-x-0 bottom-0 m-[8]",
            progress { class: "progress w-full", value: 30, max: 100 }

            div { class: "flex justify-items-center items-center w-full",
                div { width: "50px", height: "50px",
                    Icon { width: 50, height: 50, icon: HiBeaker }
                }
                div { "Song" }
                div { class: "grow flex justify-center items-center",
                    button {
                        class: "btn btn-square",
                        width: "30px",
                        height: "30px",
                        Icon { width: 30, height: 30, icon: HiArrowCircleLeft }
                    }
                    button {
                        class: "btn btn-square",
                        width: "30px",
                        height: "30px",
                        Icon { width: 30, height: 30, icon: HiPlay }
                    }
                    button {
                        class: "btn btn-square",
                        width: "30px",
                        height: "30px",
                        Icon { width: 30, height: 30, icon: HiArrowCircleRight }
                    }
                }
                input {
                    class: "range range-xs w-1/6",
                    r#type: "range",
                    min: 0,
                    max: 100,
                    value: 50,
                }
            }
        }
    )
}
