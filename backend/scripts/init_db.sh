#! /usr/bin/env bash
set -x
set -eo pipefail

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME=${POSTGRES_DB:=pdfreader}
DB_PORT="${POSTGRES_PORT:=5432}"

if [[ -z "${SKIP_DOCKER}" ]]
then
    mkdir -p /home/fredrik/.pdfreader/postgres
    chown -R fredrik:fredrik /home/fredrik/.pdfreader

    docker run \
        -v /home/fredrik/.pdfreader/postgres:/var/lib/postgresql/data \
        --user 1000:1000 \
        -v /etc/passwd:/etc/passwd:ro \
        -e PGDATA=/var/lib/postgresql/data/pgdata \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        -d postgres:alpine \
        postgres -N 1000
fi

export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d postgres -c '\q'; do
    >&2 echo "Postgres still unavailable - sleeping"
    sleep 1
done

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"
