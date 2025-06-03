# syntax=docker/dockerfile:1
# Adapted from https://docs.docker.com/guides/nodejs/containerize/

ARG NODE_VERSION=18
FROM node:${NODE_VERSION}-alpine

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

# Copy the rest of the source files into the image.
COPY . .

# Expose the port that the application listens on.
EXPOSE 3000

# Run the application.
CMD ["npm", "start"]
