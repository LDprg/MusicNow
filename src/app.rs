use std::time::Duration;

use crate::prelude::*;
use crate::timer::*;
use dioxus::{
    prelude::*,
};
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

#[derive(Clone)]
struct PlayerContext {
    pub current_track: Signal<Option<SearchTrackApi>>,
}

#[component]
fn SongItems(search: ReadSignal<SearchApi>) -> Element {
    let search = search.read();
    let items = search.collection.iter().map(move |item| {
        if let SearchElementApi::Track(track) = item {
            let new_track = track.clone();
            rsx!(
                div {
                    class: "flex m-[8]",
                    onclick: move |_| {
                        let track = new_track.clone();

                        async move {
                            let mut player = use_context::<PlayerContext>();

                            *player.current_track.write() = Some(*track);
                            Ok(())
                        }
                    },
                    if let Some(url) = &track.artwork_url {
                        img { src: url.to_string() }
                    }
                    div {
                        "{track.title}"
                        br {}
                        br {}
                        if let Some(publisher_metadata) = &track.publisher_metadata
                            && let Some(artist) = &publisher_metadata.artist
                        {
                            "{artist}"
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
    let player = use_context_provider(|| PlayerContext {
        current_track: Signal::new(None),
    });

    let mut position_sync = use_signal(|| Duration::ZERO);
    let mut position_inst = use_signal(Instant::now);
    let mut position = use_signal(|| Duration::ZERO);

    let mut duration = use_signal(|| Duration::ZERO);
    let mut is_paused = use_signal(|| true);
    let mut volume = use_signal(|| 50.0);

    use_future(move || async move {
        loop {
            sleep(Duration::from_millis(10)).await;
            if !is_paused() {
                *position.write() = position_sync() + position_inst.read().elapsed();
            } else {
                *position.write() = position_sync();
            }
        }
    });

    let search_res = use_memo(move || {
        // search_fn.value().map(|v| match v {
        //     Ok(search) => rsx!(
        //         SongItems { search }
        //     ),
        //     Err(err) => rsx!( "{err}" ),
        // })
    });

    rsx!(
        div { class: "grid grid-rows-[auto_1fr_auto] h-screen overflow-hidden",
            label { class: "input w-full mb-[8]",
                Icon { class: "h-[1em]", icon: HiSearch }
                input {
                    class: "grow",
                    r#type: "search",
                    placeholder: "Search",

                    oninput: move |event| async move {},
                }
            }

            div { class: "overflow-auto", {search_res()} }

            div { class: "m-[8]",
                progress {
                    class: "progress w-full",
                    value: position().as_millis(),
                    max: duration().as_millis(),
                }

                div { class: "flex justify-items-center items-center w-full",
                    div { width: "50px", height: "50px",
                        if let Some(track) = &*player.current_track.read()
                            && let Some(url) = &track.artwork_url
                        {
                            img { width: 50, height: 50, src: url.to_string() }
                        } else {
                            Icon { width: 50, height: 50, icon: HiBeaker }
                        }
                    }
                    div {
                        if let Some(track) = &*player.current_track.read() {
                            {track.title.clone()}
                        } else {
                            "Song"
                        }
                    }
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
                            onclick: move |_| async move {},
                            if is_paused() {
                                Icon { width: 30, height: 30, icon: HiPause }
                            } else {
                                Icon { width: 30, height: 30, icon: HiPlay }
                            }
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
                        value: volume(),
                        oninput: move |e| async move {},
                    }
                }
            }
        }
    )
}
