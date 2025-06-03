#!/bin/sh

set -e
cd /app/
diesel setup --database-url "$DATABASE_URL"
exec /bin/server
