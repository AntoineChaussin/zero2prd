#!/usr/bin/env bash

set -x
set -eo pipefail

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=postgres}"

DB_NAME="${POSTGRES_DB:=newsletter}"

DB_PORT="${POSTGRES_PORT:=5432}"
# Check if a custom host has been set, otherwise default to 'localhost'
DB_HOST="${POSTGRES_HOST:=localhost}"
export PGPASSWORD="${DB_PASSWORD}"
if psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'
then 
    echo "Postgres is already here"
else
    # Launch postgres using Docker
    docker run \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        -d postgres \
        postgres -N 1000


        # Keep pinging Postgres until it's ready to accept commands
    until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
        >&2 echo "Postgres is still unavailable - sleeping"
        sleep 1
    done
    >&2 echo "Postgres is up and running on port ${DB_PORT}!"
fi
DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
sqlx database create --database-url $DATABASE_URL

sqlx migrate run --database-url $DATABASE_URL
