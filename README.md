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
