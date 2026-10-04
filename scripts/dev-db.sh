#!/usr/bin/env bash
# ----------------------------------------------------------------------------
# Local stand-in for Supabase: PostgreSQL 17 + PostgREST 14 (the same versions
# Supabase runs), used by the integration tests of the `cashflow-data` crate.
#
# Nothing is installed system-wide: portable binaries are downloaded once into
# .dev/ (git-ignored) and the database lives in .dev/pgdata.
#
#   scripts/dev-db.sh start   start Postgres + PostgREST (creates the DB on first run)
#   scripts/dev-db.sh reset   recreate the database from supabase/schema.sql
#   scripts/dev-db.sh stop    stop both servers
#   scripts/dev-db.sh env     print the environment variables the tests need
#
# Supported hosts: macOS (arm64/x86_64) and Linux (x86_64/arm64).
# ----------------------------------------------------------------------------
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEV="$ROOT/.dev"
PG_VERSION="17.11.0"
POSTGREST_VERSION="v14.18"
PG_PORT="${CASHFLOW_DEV_PG_PORT:-54329}"
REST_PORT="${CASHFLOW_DEV_REST_PORT:-54330}"
DB_NAME="cashflow"
# Only used locally to sign test JWTs; Supabase manages its own signing keys.
JWT_SECRET="cashflow-local-development-jwt-secret-0123456789"

case "$(uname -s)-$(uname -m)" in
    Darwin-arm64)  PG_TRIPLE="aarch64-apple-darwin";       REST_ASSET="macos-aarch64.tar.xz" ;;
    Darwin-x86_64) PG_TRIPLE="x86_64-apple-darwin";        REST_ASSET="macos-x86-64.tar.xz" ;;
    Linux-x86_64)  PG_TRIPLE="x86_64-unknown-linux-gnu";   REST_ASSET="linux-static-x86-64.tar.xz" ;;
    Linux-aarch64) PG_TRIPLE="aarch64-unknown-linux-gnu";  REST_ASSET="linux-static-aarch64.tar.xz" ;;
    *) echo "Unsupported platform: $(uname -s)-$(uname -m)" >&2; exit 1 ;;
esac

PG_HOME="$DEV/postgresql-$PG_VERSION-$PG_TRIPLE"
PG_BIN="$PG_HOME/bin"
POSTGREST="$DEV/bin/postgrest"
PGDATA="$DEV/pgdata"

# The macOS PostgREST build links against Homebrew's libpq; point it at the
# libpq that ships with the portable Postgres instead.
export DYLD_LIBRARY_PATH="$PG_HOME/lib${DYLD_LIBRARY_PATH:+:$DYLD_LIBRARY_PATH}"
export LD_LIBRARY_PATH="$PG_HOME/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

psql_cmd() { PGOPTIONS="-c client_min_messages=warning" "$PG_BIN/psql" -X -q -v ON_ERROR_STOP=1 -h localhost -p "$PG_PORT" -U postgres "$@"; }

download() {
    mkdir -p "$DEV/bin"
    if [[ ! -x "$PG_BIN/postgres" ]]; then
        echo "Downloading PostgreSQL $PG_VERSION ..."
        curl -fsSL "https://github.com/theseus-rs/postgresql-binaries/releases/download/$PG_VERSION/postgresql-$PG_VERSION-$PG_TRIPLE.tar.gz" \
            | tar -xz -C "$DEV"
    fi
    if [[ ! -x "$POSTGREST" ]]; then
        echo "Downloading PostgREST $POSTGREST_VERSION ..."
        curl -fsSL "https://github.com/PostgREST/postgrest/releases/download/$POSTGREST_VERSION/postgrest-$POSTGREST_VERSION-$REST_ASSET" \
            | tar -xJ -C "$DEV/bin"
    fi
}

start_postgres() {
    if [[ ! -d "$PGDATA" ]]; then
        "$PG_BIN/initdb" -D "$PGDATA" -U postgres --auth=trust --encoding=UTF8 --no-locale >/dev/null
    fi
    if ! "$PG_BIN/pg_ctl" -D "$PGDATA" status >/dev/null 2>&1; then
        "$PG_BIN/pg_ctl" -D "$PGDATA" -l "$DEV/postgres.log" -w \
            -o "-p $PG_PORT -k $DEV -c listen_addresses=localhost" start >/dev/null
    fi
}

create_database() {
    if ! psql_cmd -d postgres -tAc "select 1 from pg_database where datname = '$DB_NAME'" | grep -q 1; then
        psql_cmd -d postgres -c "create database $DB_NAME"
        psql_cmd -d "$DB_NAME" -f "$ROOT/supabase/tests/supabase_shim.sql"
        psql_cmd -d "$DB_NAME" -f "$ROOT/supabase/schema.sql"
        psql_cmd -d "$DB_NAME" -f "$ROOT/supabase/tests/test_users.sql"
        echo "Database '$DB_NAME' created from supabase/schema.sql"
    fi
}

start_postgrest() {
    stop_postgrest
    cat > "$DEV/postgrest.conf" <<EOF
db-uri = "postgres://authenticator:authenticator@localhost:$PG_PORT/$DB_NAME"
db-schemas = "public"
db-anon-role = "anon"
jwt-secret = "$JWT_SECRET"
server-host = "127.0.0.1"
server-port = $REST_PORT
EOF
    # Started directly (not via nohup): macOS strips DYLD_* variables when
    # launching system binaries, which would hide the bundled libpq.
    "$POSTGREST" "$DEV/postgrest.conf" >"$DEV/postgrest.log" 2>&1 &
    echo $! >"$DEV/postgrest.pid"
    disown || true
    for _ in $(seq 1 50); do
        if curl -fs "http://127.0.0.1:$REST_PORT/" >/dev/null 2>&1; then return 0; fi
        sleep 0.2
    done
    echo "PostgREST did not come up, see $DEV/postgrest.log" >&2
    exit 1
}

stop_postgrest() {
    if [[ -f "$DEV/postgrest.pid" ]]; then
        kill "$(cat "$DEV/postgrest.pid")" 2>/dev/null || true
        rm -f "$DEV/postgrest.pid"
    fi
}

print_env() {
    echo "export CASHFLOW_TEST_REST_URL=http://127.0.0.1:$REST_PORT"
    echo "export CASHFLOW_TEST_JWT_SECRET=$JWT_SECRET"
    echo "export CASHFLOW_TEST_PG_URL=postgres://postgres@localhost:$PG_PORT/$DB_NAME"
}

case "${1:-start}" in
    start)
        download; start_postgres; create_database; start_postgrest
        echo "Local Supabase stand-in running (REST on http://127.0.0.1:$REST_PORT)." ;;
    reset)
        download; start_postgres; stop_postgrest
        psql_cmd -d postgres -c "drop database if exists $DB_NAME with (force)"
        create_database; start_postgrest ;;
    stop)
        stop_postgrest
        "$PG_BIN/pg_ctl" -D "$PGDATA" -m fast stop >/dev/null 2>&1 || true
        echo "Stopped." ;;
    env)
        print_env ;;
    *)
        echo "usage: $0 start|reset|stop|env" >&2; exit 2 ;;
esac
