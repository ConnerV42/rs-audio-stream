# rs-audio-stream

## Bring up the Database and run the API locally
```
docker-compose up db
cargo run
```

- Run `./scripts/init-db.sh` to standup docker-compose for postgres db connectivity
- When running locally, you can log into the admin interface with username `admin` and password `everythinghastostartsomewhere`

### Routes
- `health_check`:
```
curl -v `http://127.0.0.1:8080/health_check`
```
- `subscriptions`:
```
curl -i -X POST -d 'email=thomas_mann@hotmail.com&name=Tom' \
    http://127.0.0.1:8080/subscriptions
```
```
curl -i -X POST -d 'email=thomas_mann@hotmail.com&name=Tom' \
    https://audio-streamer.fly.dev/subscriptions
```
`login`:
```
http://127.0.0.1:8080/login
```
`home:`
```
http://127.0.0.1:8080
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
docker run -p 8080:8080 audiostreamer
```

## Postgres Database
- This application uses a Postgres database and [SQLx](https://github.com/launchbadge/sqlx).

- Generate query metadata to support offline compile-time verification in CI.
```
cargo sqlx prepare -- --lib
```
- To create a new database migration file with sqlx:
```
sqlx migrate add $MIGRATION_FILE_NAME
```
- To actually run the migrations against your local database:
```
./scripts/init-db.sh
```

## Redis Session Store
- Start Redis locally via docker-compose
```
./scripts/init-redis.sh
```

## Cargo
- Run unit and integration tests:
```
cargo test
```
- See detailed logs during test execution:
```
export RUST_LOG="sqlx=error,info"
export TEST_LOG=enabled
cargo t subscribe_fails_if_there_is_a_fatal_database_error | bunyan
```
- Isolate and view elasped test execution time in milliseconds
```
export TEST_LOG=true
cargo t --quiet --release non_existing_user_is_rejected | grep "HTTP REQUEST" | bunyan
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

The following secrets must be set as env vars on fly.io container:
```
APP_DATABASE__USERNAME="XXX"
APP_DATABASE__PASSWORD="XXX"
APP_DATABASE__HOST="XXX"
APP_DATABASE__PORT=5432
APP_DATABASE__DATABASE_NAME="XXX"
APP_APPLICATION__BASE_URL="XXX"
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

## OS considerations
If you're running on Linux, you might see errors like the one below. This is due to a limit enforced by the OS on the maximum number of open
file descriptors (including sockets) for each process.
Given that we are running all tests as part of a single binary, we might be exceeding it. The limit is usually 1024, raise it to (e.g. `ulimit -n 10000`).
```
thread 'actix-server worker 2' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 24, kind: Uncategorized, message: "Too many open files" }'
```

## Connect to Postgres Database with psql
```
psql -h localhost -p 5432 -U postgres -d postgres
SELECT * FROM users;
```
