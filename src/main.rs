//! This example is specially useful for the OAuth tests. It simply obtains an
//! access token and a refresh token with all available scopes.
//!
//! Set RSPOTIFY_CLIENT_ID, RSPOTIFY_CLIENT_SECRET and RSPOTIFY_REDIRECT_URI in
//! an .env file or export them manually as environmental variables for this to
//! work.

use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};

fn main() {
    // You can use any logger for debugging.

    // The credentials must be available in the environment. Enable the
    // `env-file` feature in order to read them from an `.env` file.
    let creds = Credentials::from_env().unwrap();

    // Using every possible scope
    let scopes = scopes!("user-library-read", "playlist-modify-public");
    let oauth = OAuth::from_env(scopes).unwrap();

    let spotify = AuthCodeSpotify::new(creds, oauth);

    let url = spotify.get_authorize_url(false).unwrap();
    // This function requires the `cli` feature enabled.
    spotify.prompt_for_token(&url).unwrap();

    let limit = 50;
    let mut offset = 0;

    let mut playlists = vec![];

    loop {
        let page = spotify
            .current_user_playlists_manual(Some(limit), Some(offset))
            .unwrap();
        for item in page.items {
            match item.public {
                Some(x) => {
                    if x {
                        playlists.push(item.id);
                    }
                }
                None => {
                    println!("bruh")
                }
            }
        }

        // The iteration ends when the `next` field is `None`. Otherwise, the
        // Spotify API will keep returning empty lists from then on.
        if page.next.is_none() {
            break;
        }

        offset += limit;
    }

    for playlist in playlists {
        spotify
            .playlist_change_detail(playlist, None, Some(false), None, None)
            .unwrap();
    }
}
