# Unihack Project

## Required Tools

```sh
# use nightly toolchain
rustup toolchain install nightly --allow-downgrade
rustup target add wasm32-unknown-unknown
cargo install --locked cargo-leptos
cargo install --locked stylance-cli
# maybe required
npm install -g sass
```

## Running

```sh
cargo leptos watch & stylance --watch .
```

## Compiling for Release

```bash
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site
