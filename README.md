# MusicNow

## Note that MusicNow is in early development stages, so expect missing features breakages and similar

## Why MusicNow?

Well music now it a reaction to Spotify's broken app (It has been broken alot for me, with login and playback issues). While SoundCloud provides an app I really don't like it (design structure features). Therefore I decided to programm MusicNow.

MusicNow features:

- Fullstack Rust from web-request to ui to audio playback and decoding
- Fast and responsive due to dioxus
- Crossplatform (Linux, Windows, Mac, IOs, Android)
- Playback without login

## How to build

Since it is a dioxus project it can be simple build and developed using the `dx` tool (see their docs for more info).

For android builds you will need to use the cli from the main branch since it updated the minimum android sdk to 28 which is required for aaudio support. This will change when dioxus 0.7 is fully released.

## Where does the music come from?

MusicNow uses SoundCloud APIs for everything currently, however there might be additional providers in the future.

## Could I use the web platform?

It is possible to use MusicNow with the web platform, although it is concidered mainly for development. Keep in mind that the frontend and backend WON'T communicate with any encryption or authentification, since they are supposed to run on the same device. So NEVER use the web platform over an unsecure or public network, you have been warned!
