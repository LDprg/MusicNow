/**
 * @typedef {{title: String, artist: String, image_url: (null | String) ,mbid: (null | String)}} Track
 */

export const ui = $state({
    menu: false,
});

/**
 * @type {{track: null | Track, tracks: Track[]}}
 */
export const player = $state({
    track: null,
    tracks: [],
});
