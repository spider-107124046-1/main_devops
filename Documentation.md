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

### 🪄 Dockerize!

