# syntax=docker/dockerfile:1
# Adapted from https://docs.docker.com/guides/rust/develop/

################################################################################
# Create a stage for building the application.

ARG RUST_VERSION=1.87
ARG APP_NAME=login-app-backend
FROM rust:${RUST_VERSION}-slim AS build
ARG APP_NAME
WORKDIR /app

# Install required packages
RUN apt-get update && apt-get install -y libpq-dev pkg-config build-essential

RUN cargo install diesel_cli --no-default-features --features postgres

# Rust dependency version issues
COPY ./Cargo.toml Cargo.toml
COPY ./Cargo.lock Cargo.lock

# Mount source code as read-only and build
RUN --mount=type=bind,source=src,target=src \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=bind,source=migrations,target=migrations \
    <<EOF

set -e
cargo update -p time
cargo build --release  
cp ./target/release/$APP_NAME /bin/server
EOF

################################################################################
# Final image, minimal runtime environment

FROM debian:bookworm-slim AS final

# libpq-dev is for development, libpq5 is for runtime, curl is for healthcheck
RUN apt-get update && apt-get install -y --no-install-recommends curl libpq5

WORKDIR /app
# Copy migration files and Diesel config for runtime migrations
COPY ./migrations migrations
COPY ./src/schema.rs src/schema.rs
COPY ./diesel.toml diesel.toml

# Entrypoint to migrate every time (to update db with code changes)
COPY ./entrypoint.sh /
RUN chmod +x /entrypoint.sh

# https://docs.docker.com/build/building/best-practices/#user
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

EXPOSE 8080

CMD ["/entrypoint.sh"]