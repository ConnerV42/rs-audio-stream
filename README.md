rs-audio-stream

## Run locally:

1. `cargo run` to start api
2. `cd` into `site` and run `python -m http.server 8000`
3. Put wav or mp3 file in the ./audio directory
3. Go to:
```
http://localhost:8080/audio/wav/{audioFileName}
http://localhost:8080/audio/mp3/{audioFileName}
http://localhost:8000 for web ui (currently broken)
```

Run `cargo doc --open` to view docs

