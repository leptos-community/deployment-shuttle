<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos Axum Starter Template <i>Hosted on Shuttle.rs</i>

Note: This Leptos starter template has been modified to deploy to [Shuttle.rs](https://www.shuttle.rs/)

For further details and for updates, see here: https://github.com/shuttle-hq/shuttle/issues/1002#issuecomment-1853661643

---

This is a template for use with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool using [Axum](https://github.com/tokio-rs/axum).

## Creating Your Template Repo

If you don't have `cargo-leptos` installed you can install it with

```sh
cargo install cargo-leptos
```

If you don't have `cargo-generate` installed already, install it with

```sh
cargo install cargo-generate
```

Then run
```sh
cargo generate --git https://github.com/Rust-WASI-WASM/shuttle-leptos-axum.git
```

to generate a new project template. Then

```sh
cd {{project-name}}
```

to go to your newly created project.

Feel free to explore the project structure, but the best place to start with your application code is in `src/app.rs`.

Addtionally, Cargo.toml may need updating as new versions of the dependencies are released, especially if things are not working after a `cargo update`.


## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future)



## Setting Up Shuttle.rs

First, make sure you have the [Shuttle.rs](https://www.shuttle.rs/) CLI: run

```sh
cargo install cargo-shuttle
```

to install the Shuttle CLI. Then, to log in to Shuttle, run

```sh
cargo shuttle login
```

When the Shuttle webpage opens, copy the API key and enter it into the Shuttle CLI.


In your project folder, run

```sh
cargo shuttle project start
```

to initiate your new project with Shuttle. At this point, you can either run your project locally using the command

```sh
cargo shuttle run --release
```
NB: `cargo shuttle run` by itself does not work - release mode must be used.


Or you can deploy to Shuttle.rs manually using the command

```sh
cargo leptos build --release && cargo shuttle deploy
```

Note: If you update the dependencies of your project, you'll need to re-initiate your Shuttle project using

```sh
cargo shuttle project restart
```

If you find that your deployed project does not look like it's updating, you may need to run the following commands in your project folder:

```sh
cargo clean
cargo leptos build --release && cargo shuttle deploy
```

## Running your Shuttle project locally

```sh
cargo shuttle run
```


## Compiling for Release
```sh
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site, and

```sh
cargo shuttle deploy
```

will deploy your updated application to [Shuttle.rs](https://www.shuttle.rs/).


## Testing Your Project
```sh
cargo leptos end-to-end
```

```sh
cargo leptos end-to-end --release
```

Cargo-leptos uses Playwright as the end-to-end test tool.
Tests are located in end2end/tests directory.


<!-- ----------- Not relevant for deploying to Shuttle ----------------

## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:

```text
shuttle-leptos
site/
```

Set the following environment variables (updating for your project as needed):

```text
LEPTOS_OUTPUT_NAME="shuttle-leptos"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```

Finally, run the server binary.

-->
