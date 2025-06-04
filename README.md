# Spider Devops Inductions - Task 2

### Tech Stack

- Language: *Rust*
- Framework: *Actix-Web*
- ORM: *Diesel*
- Database: *PostgresSQL*
- Frontend: *React*
- Compilers: *Rust v1.7.0 and node v18.0.0*

### Docker Setup

#### Prerequisites

Configure `nginx.conf` in frontend if you want to setup HTTPS.

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

Note that healthchecks are not included with the command.

### Setup with Docker Compose (Recommended)

- Copy the `.env.example` file to `.env` and edit the database credentials
- Edit the `docker-compose.yml` file
- Run `docker compose up -d --build` to build and start the containers.