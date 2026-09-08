# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A small client-side web app, built with the [Yew](https://yew.rs) framework (Rust compiled to WASM), that displays:
1. A "sobriety coin" showing years sober as a roman numeral.
2. The count of continuous sober days.

The sobriety start date is hardcoded in `src/lib.rs` (`App` component) as a `NaiveDate`. State updates every 5 seconds via a `gloo::timers::callback::Interval` so the displayed values stay current without a page reload.

## Build & run

The site is built with [Trunk](https://trunkrs.dev), not plain `cargo build`, because it needs to compile to `wasm32-unknown-unknown` and bundle `index.html`/`style.css`/assets.

One-time setup:
```
cargo install --locked trunk
rustup target add wasm32-unknown-unknown
cargo install --locked wasm-bindgen-cli
```

Development server (rebuilds on change):
```
trunk serve
```

Production build (outputs to `dist/`):
```
trunk build
```

Deployment (manual, not automated in this repo): the contents of `dist/` are synced to an S3 bucket (`aws s3 sync --delete dist/ s3://<bucket-name>`) fronted by CloudFront.

A `shell.nix` is available (`nix-shell`) that provisions all of the above (`rustup` + wasm32 target, `trunk`, `wasm-bindgen-cli`) plus the test toolchain below.

There is no linting configuration beyond the default `cargo` toolchain.

## Tests

Two separate test paths, because component rendering needs a real DOM:

- **Pure logic** (`src/lib.rs`, e.g. `roman_years_since`, `days_since`): plain `#[test]` functions, run on the host target with:
  ```
  cargo test
  ```
- **Component rendering** (`tests/components.rs`, an integration test crate): uses `wasm-bindgen-test` + `yew::Renderer::with_root_and_props` to mount a component into a detached DOM element and assert on its `inner_html()`. These must run in an actual browser via a webdriver, headless:
  ```
  wasm-pack test --headless --firefox
  ```
  (needs `wasm-pack`, `geckodriver`, and `firefox` — all included in `shell.nix`).

**Version pinning gotcha**: `wasm-bindgen`, `wasm-bindgen-cli` (the CLI binary trunk/wasm-pack invoke), and `wasm-bindgen-test` must all resolve to the *exact same patch version* — a mismatch fails loudly at build/link time (clear error naming the two schema versions), or, if `wasm-bindgen`/`wasm-bindgen-test` merely drift from each other, silently produces "no tests to run!" with zero tests executed and no error. If `cargo update` pulls a newer `wasm-bindgen`/`wasm-bindgen-test` than the `wasm-bindgen-cli` version pinned in `shell.nix` supports, pin it back down with `cargo update -p wasm-bindgen --precise <version>` (matching `wasm-bindgen --version` from the nix shell) and keep `wasm-bindgen-test`'s version equally close to it.

Component tests must live under `tests/` (integration tests), not as `#[cfg(test)] mod tests` inside `src/` — `wasm-bindgen-test`'s custom harness can't take over a `--lib`/`--bin` unit-test binary, and doing so silently yields "no tests to run!" as well.

## Architecture

- `src/main.rs` — binary entry point; just mounts the `App` component via `yew::Renderer`.
- `src/lib.rs` — the root `App` component. This is where the sobriety start date, the roman-numeral conversion, and the day-count calculation happen. `App` owns the `now` state (a `NaiveDateTime`) and passes derived values down as props (`years: String` to `Coin`, `days: i64` to `ContinuousSobriety`). Presentational components do not hold their own logic or state.
- `src/components/` — presentational (props-in, html-out) Yew function components:
  - `coin.rs` — renders the roman-numeral year count.
  - `continuous_sobriety.rs` — renders the day count.
  - `mod.rs` — re-exports the two components above; register any new component here.
- `index.html` — Trunk's entry point. Asset wiring (CSS, favicon, copied files like `background.jpg`/`image.png`) is declared here via `data-trunk` `<link>` tags, not in Rust code.
- `style.css` — global styles, including the background image treatment.

## Notable dependencies

- `roman` — pulled from the author's own git repo (`glyphrider/roman.rs`), used only for arabic→roman numeral conversion (`roman::convert::to`).
- `chrono` — date/time handling (`Local::now()`, `NaiveDate`, `NaiveDateTime`).
- `gloo` — used here specifically for its timer/interval callback to drive periodic re-renders.
- `yew` — pulled from git `main` (not crates.io), `csr` (client-side rendering) feature only.
