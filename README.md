rs-audio-stream

## Run locally:

1. `cargo run` from project root to start api
2. `cd` into `site` and run `python -m http.server 8000`
3. Put wav or mp3 file in the ./audio directory
4. Go to:
```
http://localhost:8000 (UI)
http://localhost:8080/audio/{audioFileName}/wav
http://localhost:8080/audio/{audioFileName}/mp3
```

Run `cargo doc --open` to view docs

