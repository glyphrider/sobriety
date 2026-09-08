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

There is no test suite and no linting configuration beyond the default `cargo` toolchain.

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
