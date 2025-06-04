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

