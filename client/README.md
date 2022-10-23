# Client

The client binary

## Dependencies

You'll need the most up to date version of [math](https://github.com/roadrunner-craft/math) and [assets](https://github.com/roadrunner-craft/assets)

```sh
# from ~/src/roadrunner-craft
git clone https://github.com/roadrunner-craft/math
git clone https://github.com/roadrunner-craft/assets
```

#### Export the assets

```sh
# from ~/src/roadrunner-craft
./assets/scripts/export.py ./roadruner/client
```

## Build

```sh
# from ~/src/roadruner-craft/client
cargo build [--release]
```

## Test

```sh
# from ~/src/roadruner-craft/client
cargo test [--release]
```

## Run

```sh
# from ~/src/roadruner-craft/client
cargo run [--release]
```

## Bundle

```sh
# from ~/src/roadruner-craft/client
# install the bundle command
cargo install cargo-bundle

# build for a release
cargo bundle --release
```

## Features

To enable a feature, type `cargo run --features FEATURE_NAME`. Here's the list of currently available features:

- **watchers**: watch the `res` folder to reload assets at runtime
- **remote**: temporary flag to make the client connect to the server hardcoded in `main.rs`
