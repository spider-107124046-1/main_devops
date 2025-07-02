#!/bin/sh

set -e
cd /app/
diesel setup --database-url "$DATABASE_URL"
mkdir -p /var/log/login-app-backend/

# "Observability": The following line mirrors logs to both stdout and /var/log/login-app-backend/
exec /bin/server 2>&1 | tee /var/log/login-app-backend/$(date +'%Y-%m-%d_%H-%M-%S').log
