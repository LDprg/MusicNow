use crate::prelude::*;
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
fn SongItems(search: ReadSignal<SearchApi>) -> Element {
    let search = search.read();
    let items = search.collection.iter().map(move |item| {
        if let SearchElementApi::Track(track) = item {
            let id = track.id;
            rsx!(
                div {
                    class: "flex m-[8]",
                    onclick: move |_| async move {
                        play(id).await?;
                        Ok(())
                    },
                    if let Some(url) = &track.artwork_url {
                        img { src: url.to_string() }
                    }
                    div {
                        "{track.title}"
                        br {}
                        br {}
                        if let Some(publisher_metadata) = &track.publisher_metadata {
                            if let Some(artist) = &publisher_metadata.artist {
                                "{artist}"
                            } else if let Some(user) = &track.user {
                                "{user.username}"
                            }
                        } else if let Some(user) = &track.user {
                            "{user.username}"
                        }
                    }
                }
            )
        } else {
            rsx!()
        }
    });

    rsx!(
        {items}

    )
}

#[component]
fn Home() -> Element {
    let mut search_fn = use_action(search);
    let search_res = use_memo(move || {
        search_fn.value().map(|v| match v {
            Ok(search) => rsx!(
                SongItems { search }
            ),
            Err(err) => rsx!( "{err}" ),
        })
    });

    rsx!(
        div { class: "grid grid-rows-[auto_1fr_auto] h-screen overflow-hidden",
            label { class: "input w-full mb-[8]",
                Icon { class: "h-[1em]", icon: HiSearch }
                input {
                    class: "grow",
                    r#type: "search",
                    placeholder: "Search",

                    oninput: move |event| async move { search_fn.call(event.value(), 20, 0).await },
                }
            }

            div { class: "overflow-auto", {search_res()} }

            div { class: "m-[8]",
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
                            Icon {
                                width: 30,
                                height: 30,
                                icon: HiArrowCircleLeft,
                            }
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
                            Icon {
                                width: 30,
                                height: 30,
                                icon: HiArrowCircleRight,
                            }
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
        }
    )
}
