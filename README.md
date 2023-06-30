# rs-audio-stream

## Running the API locally
```
docker-compose up db
cargo run
```

- Run ./init.sh to standup docker-compose for postgres db connectivity

### Routes
- `health_check`:
```
curl -v `http://127.0.0.1:8000/health_check`
```
- `subscriptions`:
```
curl -i -X POST -d 'email=thomas_mann@hotmail.com&name=Tom' \
    http://127.0.0.1:8000/subscriptions
```

# Docker

- Run the app and postgres database
```
docker-compose up
```
- Bring up postgres database only
```
docker-compose up db
```
- Build image
```
docker build --tag audiostreamer .
```
- Run image
```
docker run -p 8000:8000 audiostreamer
```

## Database
- This POC uses a MySQL database, and [SQLx](https://github.com/launchbadge/sqlx) as a database interface library.

Generates query metadata to support offline compile-time verification.
- `sqlx prepare -- --lib`

# Tests
- Run `cargo test`

## Additional Documentation
- Run `cargo doc --open` to view docs

