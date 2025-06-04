This document *documents*
- my approach and reasoning
- instructions to build and run the application [(frontend)](Frontend/README.md) [(backend)](Backend/README.md)
- challenges faced and solutions implemented

### Before jumping into Docker...

I tried recreating the development environment using the instructions given by the developer. While running `cargo install` with rust 1.79 (as mentioned in the instructions), this error occurred:

```package  icu_collections v2.0.0 cannot be built because it requires rustc 1.82 or newer, while the currently active rustc version is 1.70.0. Try re-running cargo install with --locked```

Running `cargo install --locked` as it says, gives a different error:

```Package diesel_cli v2.2.10 does not have feature mysqlclient-sys. It has an optional dependency with that name, but that dependency uses the "dep:" syntax in the features table, so it does not have an implicit feature with that name.```

After a back and forth with the developer, we agreed on using rust 1.87 (latest version at the time of writing) to build the app. Since we are using newer version, `--locked` is removed from the build commands. This brings a new error:

![image](https://github.com/user-attachments/assets/fb7aa0eb-91bf-43e9-835f-f05b84f0ba56)

This was fixed by running `cargo update -p time` as mentioned.

I installed postgresql (sudo apt install `postgresql` `postgresql-client`), set password, logged in, and created the `rust_server` database:

![image](https://github.com/user-attachments/assets/1d0e41f9-6567-45f9-9a77-ea6b413b3d88)

I constructed the postgres URL for the connection (`postgres://postgres:<password>@127.0.0.1:5432/rust_server`) and set it as the value of `DATABASE_URL` in `.env`

After running `diesel setup` and then `cargo run`, the backend was ready to serve

```bash
ilam@pseudoforceyt-vm:~/login-app$ curl localhost:8080
Hello, Actix Web!ilam@pseudoforceyt-vm:~/login-app$
```

Building the frontend was fairly easy, `npm i` and then `npm start` where the code resided, launched `http://localhost:3000` soon after it compiled.

![image](https://github.com/user-attachments/assets/a3511c16-adb0-4bc3-a4f0-c8ce3f5b4733)

I was greeted with the login page. Looking at `src/pages` I found that there was a page called `http://localhost:3000/register` to which I navigated and tried to create an account.

![image](https://github.com/user-attachments/assets/3df39200-b784-44dc-a9e2-c06d31d970ba)

Logging in with the same credentials greeted me with the homepage:

![image](https://github.com/user-attachments/assets/0c69a401-3daf-411a-a6e3-874f50046191)

Neat! Now that we've confirmed that the application is working, we move on to

### 🪄 Dockerizing!

The production environment needs to be as clean and lightweight as possible. The commands we used to build in the previous section need to be modified slightly to utilise the clean building options the toolchains offer.

`cargo build --release` instead of `cargo build`

`npm ci && npm run build` instead of `npm i && npm start`

The final image which is going to be serving our application does not need the building tools, so we switch to an even lighter image after the build is complete, and then copy the compiled files to it.

`docker init` (from docker desktop tools) offerred a good starting point, which I modified according to the project needs.

#### Issues encountered during Backend dockerization

1. **`diesel setup`:** In our layered build process, the final build will not have diesel_CLI. So to migrate the database everytime (this is required to update the database with new changes/migrations while preserving the old data), the [diesel_CLI binary is copied to the final image.](https://github.com/spider-107124046-1/main_devops/blob/e6dd492ea763efb3efe7eb82211ab5a5525cea02/Backend/Dockerfile#L66)
2. The server software was hardcoded to bind to the address `127.0.0.1`, which rejects connections from any other addresses (such as other containers) which will reject connections from the frontend. I had to change it to `0.0.0.0` to make it listen to all interfaces (this won't be a security concern as we are using docker! ✨ isolation ✨ As long as the port 8080 isnt exposed to the public using `-p 8080:8080`, nothing else can access it other than the containers in the same network as the backend container.)
3. Utilizing the versatility of containers, we need to separate the backend software from the database software. So we use a separate container with the postgres image for the database. But, the migration step which we did at our leisure in the development environment now needs to be properly timed. While deploying the container using docker commands, we have to make sure that the database (db) container is started first, and then the backend after ensuring that db is up and running. For this, we can use a feature called **Healthcheck** which uses a periodic (or one-time, your wish) command to check if the container is "healthy". This status can then be read by other containers to accordingly perform their operations. In docker compose, each service can have a `healthcheck:` section defined, and the service depending on the health of this service can have a `depends_on:` section with the `condition` that the service is "healthy". Refer to the [docker-compose.yml](docker-compose.yml) file for clarity.

#### Issues encountered during Frontend dockerization

1. The development proxy mentioned in package.json works fine for a development environment. But here, the backend runs in a different container and `localhost:8080` will not be available. For fixing this, I switched the frontend's final image to nginx, and setup a reverse proxy for `backend:8080` that forwards **only the API call endpoints** to the backend container. Why? because the frontend isnt coded in a way such that api calls are made to a separate /api/ endpoint. Instead, they are all made in the assumption that the API is being hosted by the root domain itself. So if i forward / to `backend:8080`, the website won't be accessible, only the API will be.

Now that we've got our images to build and function as intended, we move on to make our lives a bit easier:

### Writing a Compose Configuration

`docker init` also creates a sample `compose.yaml` file which gives us the base for the backend. For the frontend, which is an nginx server now, there are no environment variables required, just the site config. Even that is copied to the image as part of the build process. The env file for the backend and the db is unified, by just taking the postgres username, password and database name separate arguments (in our case, the database **must** be named `rust_server` as the code expects that. This could be changed by the developer.) The parameters are then passed separately to the postgres container (as it requires), and constructed as a URL to be passed to the backend container (as it requires). A simple healthcheck using the `curl` command to check if the service is responding, is included for both the frontend and backend.

Now if we run `docker compose up -d --build`, all our services, networks and volumes will be automatically created and started. This concludes our dockerization. Additional setup options are provided in the comments of `docker-compose.yml` for things like HTTPS.

### Pushing the image to Docker Hub

We can build the image using `docker build -t <docker hub username>/<repo name>` to push the image to the Docker Hub image registry. The [frontend](https://hub.docker.com/r/pseudopanda/login-app-frontend/tags) and [backend](https://hub.docker.com/r/pseudopanda/login-app-backend/tags) images are available on Docker Hub at `pseudopanda/login-app-frontend` and `pseudopanda/login-app-backend`. These images can now be used in the Docker commands or Docker Compose files, instead of the built images or the `build:` section of the services. Example docker compose using the docker hub images:

```yaml
# Project name, this will be used to prefix all container names
name: authentication-app # Name found in Frontend

services:
  backend:
    image: pseudopanda/login-app-backend:latest
    depends_on:
      db:
        condition: service_healthy
### rest of the config ###
  frontend:
    image: pseudopanda/login-app-frontend:latest
    ports:
      # Change the port if needed
      # if you are using an external reverse proxy, you can comment the ports section
      - "3000:3000"
### rest of the config ###
```
