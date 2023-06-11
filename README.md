POC for audio server

# To run locally:

1. `brew install lame`
2. `find /opt/homebrew -name libmp3lame.dylibi`
3. Paste in your build.rs
4. `cargo run` to start the server
5. `cd` into `site` and run `python -m http.server 8000`

Probably will switch from LAME to ffmpeg at some point

Run `cargo doc --open` to view docs

