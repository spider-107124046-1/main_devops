#!/bin/sh

set -e

# Parse values from DATABASE_URL
# Example: postgres://user:password@host:port/dbname

DB_HOST=$(echo "$DATABASE_URL" | sed -E 's|.*://[^@]*@([^:/]+):[0-9]+/.*|\1|')
DB_PORT=$(echo "$DATABASE_URL" | sed -E 's|.*://[^@]*@[^:]+:([0-9]+)/.*|\1|')
DB_USER=$(echo "$DATABASE_URL" | sed -E 's|.*://([^:]+):.*@.*|\1|')

# Fallbacks if parsing fails
DB_HOST=${DB_HOST:-db}
DB_PORT=${DB_PORT:-5432}
DB_USER=${DB_USER:-postgres}

# https://www.postgresql.org/docs/current/app-pg-isready.html
until pg_isready -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER"; do
  echo "Waiting for database connection..."
  sleep 1
done

cd /app/
diesel setup --database-url "$DATABASE_URL"
exec /bin/server
