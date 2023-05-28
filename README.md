# privatise-spotify-playlists
rust rewrite of [seykuyinu/privatise-spotify-playlists](https://github.com/seykuyinu/privatise-spotify-playlists/)
Simple program to make all spotify playlists private

## Setup
- Follow the instructions [here](https://developer.spotify.com/documentation/web-api/concepts/apps) to register your app with Spotify.
- Export the env variables or add them to .env file
```bash
export RSPOTIFY_CLIENT_ID=your-client-id
export RSPOTIFY_CLIENT_SECRET=your-client-secret
export RSPOTIFY_REDIRECT_URI=http://localhost:8888/callback
```

## Usage
1. `cargo run`
2. It will open a browser window to authorize app from spotify
3. Copy the localhost url it redirects to and paste it in terminal
4. Done!

## TODO
- [ ] Add Refresh token (realistically you will only run this program once but in case you made your playlists public again lol)
- [ ] ???
