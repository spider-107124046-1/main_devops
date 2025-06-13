# Spider Devops Inductions - Task 2

### [Read the documentation here](Documentation.md)

### Tech Stack

- Language: *Rust*
- Framework: *Actix-Web*
- ORM: *Diesel*
- Database: *PostgresSQL*
- Frontend: *React*
- Compilers: *Rust v1.7.0 and node v18.0.0*

### Docker Setup

#### Prerequisites

If you want to use HTTP (or use an external reverse proxy for HTTPS), you don't need to take any action on `nginx.conf`. 

If you want to use an existing certificate to host the site on HTTPS, copy `nginx-https.conf` to `nginx.conf`, and **change "yourdomain.com" to your actual domain in both `nginx.conf` and `docker-compose.yml`.**

```bash
docker network create authentication-app_default
docker volume create db_data
cd Frontend # pwd: <repo_name>/Frontend/
docker build -t authentication-app-frontend .
cd ../Backend # pwd: <repo_name>/Backend/
docker build -t authentication-app-backend .

export POSTGRES_USER=myuser
export POSTGRES_PASSWORD=mypassword
export POSTGRES_DB=mydb
```

#### PostgreSQL Container

```bash
docker run -d \
  --name authentication-app_db \
  --network authentication-app_default \
  -e POSTGRES_USER \
  -e POSTGRES_PASSWORD \
  -e POSTGRES_DB \
  -v db_data:/var/lib/postgresql/data \
  postgres:16
```

#### Backend

```bash
docker run -d \
  --name authentication-app-backend \
  --network authentication-app_default \
  # -p 8080:8080 # if you want to expose the API to the public \ 
  -e DATABASE_URL=postgresql://${POSTGRES_USER:-postgres}:${POSTGRES_PASSWORD}@authentication-app_db:5432/${POSTGRES_DB:-rust_server} \
  authentication-app-backend
```

#### Frontend

```bash
docker run -d \
  --name authentication-app-frontend \
  --network authentication-app_default \
  -p 3000:3000 \
  authentication-app-frontend
```

Note that healthchecks and TLS certificate mounting are not included with the command. **Use Docker Compose method for more control.**

### Setup with Docker Compose (Recommended)

- Copy the `.env.example` file to `.env` and edit the database credentials
- Edit the `docker-compose.yml` file
- Run `docker compose up -d --build` to build and start the containers.

### Access the Services

By default, the frontend service is exposed to `<server-ip>:3000`. You can access both the frontend and backend by visiting `http://<server-ip>:3000`. \[For instance, if you are using the browser on the same computer on which docker is running, `http://localhost:3000`\]

### Troubleshooting

#### The login and register pages load, but on clicking login or register, nothing happens.

1. Check the logs of the frontend container: `docker logs authentication-app-frontend-1` (replace with your container name)
2. Identify the `[error]`. It is most likely failing to connect to the backend
3. Check if your environment file is correctly configured
4. Check if the backend container is running. If not, run it with `docker compose up -d` or the specified docker command
5. Check your docker compose and nginx configuration

#### User not authenticated, even with correct password

![image](https://github.com/user-attachments/assets/d9296867-3b32-405b-a756-d10bc3b0958f)

1. Ensure that all 3 containers are running (`docker ps`). If not, start them with `docker compose up -d`
2. Check if your environment file is correctly configured
3. Check whether the frontend container is able to connect to the backend. To do this, run `docker exec -it authentication-app-frontend-1 curl backend:8080` (replace with your container name). The command should return "Hello, Actix Web!": <br><br> ![image](https://github.com/user-attachments/assets/b3c093c7-d204-419a-9081-7c6982893fd8) <br> If not, check if both containers are in the same network, and add them to the same network. `docker container inspect authentication-app-frontend-1 | jq '.[0].NetworkSettings.Networks'` [or `docker inspect --format='{{json .NetworkSettings.Networks}}' authentication-app-frontend-1`] (replace with your container names) <br><br> ![image](https://github.com/user-attachments/assets/c225dabc-d8da-41f3-bedd-285c8e57cbb1) <br> This should not be a problem with the default docker compose configuration.<br><br>
4. If the frontend is able to connect to the backend, check whether the backend container is able to connect to the frontend. `docker exec -it authentication-app-backend-1 curl frontend:3000` <br><br> ![image](https://github.com/user-attachments/assets/c71c2b8b-8bb4-485d-8f0e-9ceda82c7b41) <br> If not, check for any misconfigured firewall rules and remove them (`iptables -L`, `nft list ruleset`,  `ufw status numbered`).<br><br>
5. Check if the backend is able to connect to the database: `docker logs authentication-app-backend-1`. Database errors such as:

```log
[2025-06-04T14:17:09Z ERROR r2d2] server closed the connection unexpectedly
    	This probably means the server terminated abnormally
    	before or while processing the request.
    
[2025-06-04T14:17:09Z ERROR r2d2] could not translate host name "db" to address: Temporary failure in name resolution
```
indicate that the backend isnt able to connect to the database. Check if your environment file is correctly configured.

#### 500/502 Error Code on frontend, or Frontend not loading

Check your nginx.conf for any syntax errors or misconfigration, then run `docker compose up -d --build` again. If that did not help, try all the above troubleshooting steps.
