# rs-audio-stream

## Run locally:

- Run ./init.sh to standup docker-compose for postgres db connectivity
- curl -v http://127.0.0.1:8000/health_check

## Docker
- Running `docker-compose up db` brings up just the database locally
- Build Dockerfile with `docker build --tag audiostreamer .`
- Run image with `docker run -p 8000:8000 audiostreamer`

## Database
- This POC uses a MySQL database, and [SQLx](https://github.com/launchbadge/sqlx) as a database interface library.

Generates query metadata to support offline compile-time verification.
- `sqlx prepare -- --lib`

# Tests
- Run `cargo test`

## Additional Documentation
- Run `cargo doc --open` to view docs

