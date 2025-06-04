# syntax=docker/dockerfile:1
# Adapted from https://docs.docker.com/guides/nodejs/containerize/

################################################################################
# Create a stage for building the site
ARG NODE_VERSION=18
FROM node:${NODE_VERSION}-alpine AS build

# Use production node environment by default.
ENV NODE_ENV=production

WORKDIR /usr/src/app

# Again, builds failing because of readonly files. Using COPY instead of --mount
COPY . .

RUN chown -R node:node /usr/src/app

# Run the application as a non-root user.
USER node

RUN npm ci

RUN npm run build

################################################################################
# Final image, minimal runtime environment
FROM nginx:alpine AS final

# Copy custom nginx config
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Copy static build output from build stage
COPY --from=build /usr/src/app/build /usr/share/nginx/html

EXPOSE 3000

# Entrypoint inherited from nginx image