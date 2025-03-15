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

## Fetching data
```sh
# Fetch data
wget https://github.com/blorbb/unihack
-2025/archive/6004e9e2661abda0f0778b297f3deb53271e1cd7/unihack-2025
-6004e9e2661abda0f0778b297f3deb53271e1cd7.tar.gz
# Extract
tar --extract --file unihack-2025-6004e9e2661abda0f0778b297f3deb53271e1cd7.tar.gz
# Rename directory
mv unihack-2025-6004e9e2661abda0f0778b297f3deb53271e1cd7/ class-data/
```

```sh
tree -L 1
.
├── backend
├── Cargo.lock
├── Cargo.toml
├── class-data
├── LICENSE
├── public
├── README.md
├── rust-toolchain.toml
├── scraper
├── src
├── style
├── target
└── test-backend
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

## Dev Notes

### CSS

`global.module.scss` contains global styles, this should not have any classes. Every component has a corresponding `.module.scss` file, which should be imported in Rust with `stylance::import_style!(s, "componentname.module.scss");`.

**Transitions**

Since transitions don't compose when overridden, there are two 'higher level' transitions:

- `--base-transitions` defined by global styles.
- `--component-transitions` defined by a component. This is equal to `--base-transitions` by default.

If a component is given a class that overrides transitions, the transition should have `var(--component-transitions)`.

```scss
* {
  transition: var(--component-transitions), filter var(--time-transition);
}
```

If a component adds transitions, it should set `--component-transitions: var(--base-transitions), background var(--time-transition)` etc.
