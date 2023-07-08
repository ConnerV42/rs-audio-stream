# rs-audio-stream

## Bring up the Database and run the API locally
```
docker-compose up db
cargo run
```

- Run `./init-db.sh` to standup docker-compose for postgres db connectivity

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
```
curl -i -X POST -d 'email=thomas_mann@hotmail.com&name=Tom' \
    https://audio-streamer.fly.dev/subscriptions
```

## Docker

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
- This application uses a Postgres database and [SQLx](https://github.com/launchbadge/sqlx).

- Generate query metadata to support offline compile-time verification in CI.
```
sqlx prepare -- --lib
```

## Cargo
- Run unit and integration tests:
```
cargo test
```
- Generate and view docs:
```
cargo doc --open
```

## Deploying through fly.io 

- Build Dockerfile into imagei and deploy to production
```
fly deploy
```

- Set secrets directly in fly app:
```
fly secrets set DATABASE_URL=postgres://example.com/mydb 
```

### Locally run migrations against fly.io db:

- Forward the server port to your local system with [fly proxy](https://fly.io/docs/postgres/connecting/connecting-with-flyctl/)
```
fly proxy 5432 -a audio-streamer-postgres
```
- Then, you should be able to run your migrations:
```
DATABASE_URL=postgres://postgres:<password>@localhost:5432 sqlx migrate run
```

## Running python server
- cd into `site` and run
```
python -m http.server 8000
```


