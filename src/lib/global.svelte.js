/**
 * @typedef {{title: String, artist: String, image_url: (null | String) ,mbid: (null | String), available: boolean}} Track
 */

export const ui = $state({
    menu: false,
});

/**
 * @type {{query: null | String, track: null | Track, tracks: Track[]}}
 */
export const player = $state({
    query: null,
    track: null,
    tracks: [],
});
