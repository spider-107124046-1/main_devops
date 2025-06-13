### Building steps for the project:

(Tested on Rust 1.87)

```bash
cargo install diesel_cli --no-default-features --features postgres
cargo update -p time # Build fails due to time having breaking API changes
cargo build
```

Compiled executable `login-app-backend` will be available to run inside the `target` directory.

### Running:

Ensure that the PostgreSQL server is setup and running, and the credentials are saved to .env in the form of a postgres URI. Then run:

```bash
# in the root of the Backend/ folder
diesel setup # to migrate tables
```

For testing, use `cargo run`. For production, execute the compiled binary (built using the aforementioned steps) in the `target` directory.
