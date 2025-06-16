#!/bin/sh

# Create new ssl certificate if it does not exist
if [ ! -f /etc/ssl/certs/frontend.pem ]; then
  echo "Creating new SSL certificate..."
  openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
    -keyout /etc/ssl/private/frontend.key \
    -out /etc/ssl/certs/frontend.pem \
    -subj "/C=XX/ST=Default/L=Default/O=Default/CN=default"
else
  echo "SSL certificate already exists."
fi

# nginx alpine default entrypoint
set -e
exec nginx -g 'daemon off;'