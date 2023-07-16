#!/usr/bin/env bash

set -x # shows command output

set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  echo >&2 "Use: brew install postgresql"
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

if [[ -z "${SKIP_DOCKER}" ]]
then
    # Bring up database and api
    docker-compose --env-file .env up db -d
fi

# Ping database until it is up
export PGPASSWORD="password"
until psql -h "${POSTGRES_HOST}" -U "${POSTGRES_USER}" -p "${POSTGRES_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "***** Postgres is still unavailable - sleeping for 5 seconds *****"
  sleep 5
done
>&2 echo "***** Postgres is up and running on port ${POSTGRES_PORT} *****"

DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}
export DATABASE_URL
echo $DATABASE_URL

>&2 echo "***** Running migrations with sqlx *****"
sqlx database create
sqlx migrate run
