### Building steps for the project:

(Tested on Rust 1.87)

```bash
cargo install diesel_cli --no-default-features --features postgres
cargo update -p time # Build fails due to time having breaking API changes
cargo build
```

Compiled executable `login-app-backend` will be available to run inside the `target` directory.