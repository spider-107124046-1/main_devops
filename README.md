# Spider Devops Inductions - Task 2

### [Read the documentation here](Documentation.md)

### Tech Stack

- Language: *Rust*
- Framework: *Actix-Web*
- ORM: *Diesel*
- Database: *PostgresSQL*
- Frontend: *React*
- Compilers: *Rust v1.7.0 and node v18.0.0*

### Setup with Docker Compose (Recommended)

1. Clone the repository (easiest) or Copy the files `docker-compose.yml`, `.env.example`, `Frontend/nginx.conf`, and optionally, `Frontend/gzip.conf` (to enable gzip compression) and `Frontend/blacklist.conf` (to blacklist IPs) \[See comments in [docker-compose.yml](docker-compose.yml) for more info.\].
2. Copy the `.env.example` file to `.env` and edit the database credentials
3. Edit the nginx configuration to set domain
4. Edit the `docker-compose.yml` file, make sure to match the nginx configuration wherever applicable (for instance, the path to the SSL certificate and private key).
5. Run `docker compose up -d` to start the containers.

Alternatively, you may clone the repository and build the image yourself by using docker-compose-build.yml instead of docker-compose.yml, as opposed to pulling the image from Docker Hub.

### Docker Setup

#### Prerequisites

**This service enforces HTTPS!**

If you want to use an external reverse proxy for HTTPS, **modify the supplied `nginx.conf` to use a different port and remove the SSL lines**. 

If you want to use an existing certificate to host the site on HTTPS, mount your certificate and private key at `/etc/ssl/certs/frontend.pem` and `/etc/ssl/certs/frontend.key` respectively.

If you don't supply a certificate, by default, the frontend container generates a new self-signed certificate on startup.

```bash
docker network create login-app_default
docker volume create db_data
# You can also use the registry images at pseudopanda/login-app-frontend and pseudopanda/login-app-backend
cd Frontend # pwd: <repo_name>/Frontend/
docker build -t login-app-frontend .
cd ../Backend # pwd: <repo_name>/Backend/
docker build -t login-app-backend .

export POSTGRES_USER=myuser
export POSTGRES_PASSWORD=mypassword
export POSTGRES_DB=mydb
```

#### PostgreSQL Container

```bash
docker run -d \
  --name login-app_db \
  --restart always \
  -e POSTGRES_USER="$POSTGRES_USER" \
  -e POSTGRES_PASSWORD="$POSTGRES_PASSWORD" \
  -e POSTGRES_DB="$POSTGRES_DB" \
  -v db_data:/var/lib/postgresql/data \
  --health-cmd="pg_isready -U $POSTGRES_USER -d $POSTGRES_DB" \
  --health-interval=10s \
  --health-timeout=5s \
  --health-retries=5 \
  postgres:16
```

#### Backend

```bash
docker run -d \
  --name login-app-backend \
  --restart always \
  --link login-app_db:db \
  -e DATABASE_URL="postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@db:5432/${POSTGRES_DB}" \
  --health-cmd="curl -f http://localhost:8080/ || exit 1" \
  --health-interval=90s \
  --health-timeout=30s \
  --health-retries=5 \
  --health-start-period=30s \
  pseudopanda/login-app-backend:latest # or simply login-app-backend if you built the image
```

#### Frontend

```bash
docker run -d \
  --name login-app-frontend \
  --restart always \
  -p 443:443 \
  -p 80:80 \
  --link login-app-backend:backend \
  -v "$(pwd)/Frontend/nginx.conf:/etc/nginx/conf.d/frontend.conf:ro" \
  -v <PATH TO SSL CERTIFICATE>:/etc/ssl/certs/frontend.pem \
  -v <PATH TO SSL PRIVATE KEY>:/etc/ssl/private/frontend.key \
  -v "$(pwd)/Frontend/gzip.conf:/etc/nginx/conf.d/gzip.conf:ro" \
  --health-cmd="curl -kf https://localhost/ || exit 1" \
  --health-interval=90s \
  --health-timeout=30s \
  --health-retries=5 \
  --health-start-period=30s \
  pseudopanda/login-app-frontend:latest # or simply login-app-frontend if you built the image
```

### Access the Services

By default, the frontend service is exposed to `<server-ip>:443` (and a redirect to https on port 80). You can access both the frontend and backend by visiting `https://<server-ip>`. \[For instance, if you are using the browser on the same computer on which docker is running, `https://localhost`. This may bring up certificate issues if you are using the default configuration.\]

### Troubleshooting

#### The login and register pages load, but on clicking login or register, nothing happens.

1. Check the logs of the frontend container: `docker logs login-app-frontend-1` (replace with your container name)
2. Identify the `[error]`. It is most likely failing to connect to the backend
3. Check if your environment file is correctly configured
4. Check if the backend container is running. If not, run it with `docker compose up -d` or the specified docker command
5. Check your docker compose and nginx configuration

#### User not authenticated, even with correct password

![image](https://github.com/user-attachments/assets/d9296867-3b32-405b-a756-d10bc3b0958f)

1. Ensure that all 3 containers are running (`docker ps`). If not, start them with `docker compose up -d`
2. Check if your environment file is correctly configured
3. Check whether the frontend container is able to connect to the backend. To do this, run `docker exec -it login-app-frontend-1 curl backend:8080` (replace with your container name). The command should return "Hello, Actix Web!": <br><br> ![image](https://github.com/user-attachments/assets/b3c093c7-d204-419a-9081-7c6982893fd8) <br> If not, check if both containers are in the same network, and add them to the same network. `docker container inspect login-app-frontend-1 | jq '.[0].NetworkSettings.Networks'` [or `docker inspect --format='{{json .NetworkSettings.Networks}}' login-app-frontend-1`] (replace with your container names) <br><br> ![image](https://github.com/user-attachments/assets/c225dabc-d8da-41f3-bedd-285c8e57cbb1) <br> This should not be a problem with the default docker compose configuration.<br><br>
4. If the frontend is able to connect to the backend, check whether the backend container is able to connect to the frontend. `docker exec -it login-app-backend-1 curl -k frontend:443` <br><br> ![image](https://github.com/user-attachments/assets/c71c2b8b-8bb4-485d-8f0e-9ceda82c7b41) <br> If not, check for any misconfigured firewall rules and remove them (`iptables -L`, `nft list ruleset`,  `ufw status numbered`).<br><br>
5. Check if the backend is able to connect to the database: `docker logs login-app-backend-1`. Database errors such as:

```log
[2025-06-04T14:17:09Z ERROR r2d2] server closed the connection unexpectedly
    	This probably means the server terminated abnormally
    	before or while processing the request.
    
[2025-06-04T14:17:09Z ERROR r2d2] could not translate host name "db" to address: Temporary failure in name resolution
```
indicate that the backend isnt able to connect to the database. Check if your environment file is correctly configured.

#### 500/502 Error Code on frontend, or Frontend not loading

Check your nginx.conf for any syntax errors or misconfigration, then run `docker compose up -d --build` again. If that did not help, try all the above troubleshooting steps.
