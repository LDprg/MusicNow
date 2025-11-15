use std::time::Duration;

use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::hi_solid_icons::*};
use tokio::time::Instant;
use tokio::time::sleep;

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
    pub current_track: Signal<Option<ReleaseApi>>,
    pub current_thumb: Signal<Option<ImageApi>>,
}

#[component]
fn SongItems(search: ReadSignal<SearchReleaseApi<ReleaseApi>>) -> Element {
    let search = search.read();
    let items = search.releases.iter().map(move |item| {
        let item = item.clone();

        let image = use_resource(move || async move {
            let coverartarchive = CoverArtArchiveApi::default();
            coverartarchive.fetch_image(item.id).await
        });

        rsx!(
            div {
                class: "flex m-[8]",
                onclick: move |_| {
                    let item = item.clone();
                    async move {
                        let mut player = use_context::<PlayerContext>();

                        // let soundcloud = SoundCloudApi::default();
                        // let track_id = soundcloud.search("".to_string(), 1, 0).await?;

                        // let audio_player = AudioPlayer::default();
                        // audio_player.play(track_id).await.inspect_err(|e| error!("{}", e))?;

                        *player.current_track.write() = Some(item);
                        // *player.current_thumb.write() = image.ok();

                        Ok(())
                    }
                },
                if let Some(image) = &*image.read() && let Ok(image) = image {
                    img {
                        width: 250,
                        height: 250,
                        src: image.thumbnails.s250.to_string(),
                    }
                }
                div {
                    "{item.title}"
                    br {}
                    br {}
                    {
                        item.artist_credit
                            .iter()
                            .map(|i| i.name.clone())
                            .collect::<Vec<String>>()
                            .join(" & ")
                    }
                }
            }
        )
    });

    rsx!(
        {items}

    )
}

#[component]
fn Home() -> Element {
    let player = use_context_provider(|| PlayerContext {
        current_track: Signal::new(None),
        current_thumb: Signal::new(None),
    });

    let mut position_sync = use_signal(|| Duration::ZERO);
    let mut position_inst = use_signal(Instant::now);
    let mut position = use_signal(|| Duration::ZERO);

    let mut duration = use_signal(|| Duration::ZERO);
    let mut volume = use_signal(|| 50.0);
    let mut is_playing = use_signal(|| false);

    use_future(move || async move {
        loop {
            sleep(Duration::from_millis(10)).await;
            if is_playing() {
                *position.write() = position_sync() + position_inst.read().elapsed();
            } else {
                *position.write() = position_sync();
            }
        }
    });

    use_future(move || async move {
        let audio_player = AudioPlayer::default();

        loop {
            sleep(Duration::from_secs(10)).await;
            audio_player.update_postion();
        }
    });

    use_future(move || async move {
        let lastfm = LastFM::default();
        lastfm.login().await;
    });

    use_future(move || async move {
        let audio_player = AudioPlayer::default();

        dioxus::core::spawn(async move {
            let mut recv = audio_player.is_playing;
            while recv.changed().await.is_ok() {
                is_playing.set(*recv.borrow());
            }
        });

        dioxus::core::spawn(async move {
            let mut recv = audio_player.position;
            while recv.changed().await.is_ok() {
                position_sync.set(*recv.borrow());
                position_inst.set(Instant::now());
            }
        });

        dioxus::core::spawn(async move {
            let mut recv = audio_player.duration;
            while recv.changed().await.is_ok() {
                duration.set(*recv.borrow());
            }
        });

        dioxus::core::spawn(async move {
            let mut recv = audio_player.volume;
            while recv.changed().await.is_ok() {
                volume.set(*recv.borrow());
            }
        });
    });

    let mut search_fn = use_action(move |query: String, limit, offset| async move {
        let musicbrainz = MusicBrainzApi::default();
        musicbrainz
            .search::<ReleaseApi>(query.clone(), limit, offset)
            .await
    });
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

                    oninput: move |event| async move {
                        search_fn.cancel();
                        search_fn.call(event.value(), 20, 0).await
                    },
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
                        if let Some(url) = &*player.current_thumb.read() {
                            img {
                                width: 50,
                                height: 50,
                                src: url.image.to_string(),
                            }
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
                            onclick: move |_| {
                                let audio_player = AudioPlayer::default();
                                if is_playing() { audio_player.pause() } else { audio_player.resume() }
                            },
                            if is_playing() {
                                Icon { width: 30, height: 30, icon: HiPlay }
                            } else {
                                Icon { width: 30, height: 30, icon: HiPause }
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
                        oninput: move |e| {
                            let audio_player = AudioPlayer::default();
                            audio_player.set_volume(e.value().parse::<f64>().unwrap());
                        },
                    }
                }
            }
        }
    )
}
