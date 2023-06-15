#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
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

# Export env vars
set -o allexport
source .env
set +o allexport

docker-compose --env-file .env up -d

export PGPASSWORD="password"
until psql -h "${POSTGRES_HOST}" -U "${POSTGRES_USER}" -p "${POSTGRES_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "***** Postgres is still unavailable - sleeping for 5 seconds *****"
  sleep 5
done
>&2 echo "***** Postgres is up and running on port ${POSTGRES_DB} - running migrations now! *****"

DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}
export DATABASE_URL

sqlx database create
sqlx migrate run

# Just gotta create the migration now, but that's for another day
