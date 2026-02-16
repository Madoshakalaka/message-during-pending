# Lazy Component State Bug — Minimal Reproduction

Demonstrates that stateful hooks (`use_state`, `use_reducer`, etc.) are
broken inside lazily-loaded components created with `declare_lazy_component!`
from [yew PR #3932](https://github.com/yewstack/yew/pull/3932).

Pinned to commit [`3cf0e6e`](https://github.com/WorldSEnder/yew/commit/3cf0e6e2758a090abab0065f05015bdaa55132f4)
on the `split-wasm` branch of `WorldSEnder/yew`.

## The bug

A `Counter` component using `use_state` is rendered two ways side-by-side:

- **Normal** — works correctly, clicking "+1" increments the count.
- **Lazy** (via `declare_lazy_component!`) — renders once showing `0`,
  then **clicking "+1" does nothing**. The counter is frozen.

## Prerequisites

- Rust nightly (`rustup toolchain install nightly`)
- `wasm32-unknown-unknown` target (`rustup target add wasm32-unknown-unknown`)
- [`wasm-bindgen-cli`](https://crates.io/crates/wasm-bindgen-cli) matching the version used by yew (0.2.106):
  ```bash
  cargo install wasm-bindgen-cli --version 0.2.106
  ```
- [`wasm-opt`](https://crates.io/crates/wasm-opt):
  ```bash
  cargo install wasm-opt
  ```
- `wasm_split_cli` from the [wasm-split-prototype](https://github.com/WorldSEnder/wasm-split-prototype) repo:
  ```bash
  cargo install --git https://github.com/WorldSEnder/wasm-split-prototype \
      --features build-binary wasm_split_cli_support
  ```
