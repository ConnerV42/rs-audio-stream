#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  echo >&2 "Use: brew install postgresql (MacOS only)"
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install --version='~0.6' sqlx-cli \
--no-default-features --features rustls,postgres"
  echo >&2 "to install it."
  exit 1
fi

# Show env vars
grep -v '^#' .env

# Set environment variables that are included in the .env
set -o allexport
source .env
set +o allexport

# Bring up database and api
docker-compose --env-file .env up -d

# Ping database until it is up
export PGPASSWORD="password"
until psql -h "${POSTGRES_HOST}" -U "${POSTGRES_USER}" -p "${POSTGRES_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "***** Postgres is still unavailable - sleeping for 5 seconds *****"
  sleep 5
done
>&2 echo "***** Postgres is up and running on port ${POSTGRES_DB} - running migrations now! *****"

DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}
export DATABASE_URL
echo $DATABASE_URL

sqlx database create
sqlx migrate run
