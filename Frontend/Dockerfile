# syntax=docker/dockerfile:1
# Adapted from https://docs.docker.com/guides/nodejs/containerize/

################################################################################
# Create a stage for building the site
ARG NODE_VERSION=18
FROM node:${NODE_VERSION}-alpine AS build

# Use production node environment by default.
ENV NODE_ENV=production

WORKDIR /usr/src/app

# Change ownership of node_modules to node user
RUN chown -R node:node /usr/src/app

# Run the application as a non-root user.
USER node

# Download dependencies as a separate step to take advantage of Docker's caching.
# Equivalent of npm i from instructions
RUN --mount=type=bind,source=package.json,target=package.json \
    --mount=type=bind,source=package-lock.json,target=package-lock.json \
    --mount=type=cache,target=/root/.npm \
    npm ci --omit=dev

# Build the site
RUN --mount=type=bind,source=. \
    --mount=type=cache,target=/root/.npm \
    npm run build

################################################################################
# Final image, minimal runtime environment
FROM nginx:alpine AS final

# Copy custom nginx config
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Copy static build output from build stage
COPY --from=build /usr/src/app/dist /usr/share/nginx/html

EXPOSE 3000

# Entrypoint inherited from nginx image