#!/bin/bash
set -e

function wait_for_psql() {
  until pg_isready -h "$PSQL_HOST" -p "$PSQL_PORT" -U "$PSQl_USER" -d "$PSQL_DB"; do
    echo "Waiting for Postgres at $PSQL_HOST:$PSQL_PORT..."
    sleep 2
  done
}

wait_for_psql

echo "Database is up..."
exec "$@"
