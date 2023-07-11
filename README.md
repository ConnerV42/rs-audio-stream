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

## Postgres Database
- This application uses a Postgres database and [SQLx](https://github.com/launchbadge/sqlx).

- Generate query metadata to support offline compile-time verification in CI.
```
sqlx prepare -- --lib
```
- To create a new database migration file with sqlx:
```
sqlx migrate add add_status_to_subscriptions
```
- To actually run the migrations against your local database:
```
./scripts/init-db.sh
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

## Running python server
- cd into `site` and run
```
python -m http.server 8000
```

## OS considerations
If you're running on Linux, you might see errors like the one below. This is due to a limit enforced by the OS on the maximum number of open#[tokio::test]
async fn confirmations_without_token_are_rejected_with_a_400() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = reqwest::get(&format!("{}/subscriptions/confirm", app.address))
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status().as_u16(), 400);
} file descriptors (including sockets) for each process.
Given that we are running all tests as part of a single binary, we might be exceeding it. The limit is usually 1024, raise it to (e.g. `ulimit -n 10000`).
```
thread 'actix-server worker 2' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 24, kind: Uncategorized, message: "Too many open files" }'
```
