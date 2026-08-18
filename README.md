# test_rustbook

A small Rust project that demonstrates building and previewing UI components with [GPUI](https://github.com/zed-industries/zed) and its component library, using the `gpui_preview` tooling.

## Components

- **Button** — A customizable button with label, disabled state, and adjustable corner radius.
- **Label** — A text label styled with the theme's secondary foreground color.

Both components are marked with the `#[derive(Previewable)]` macro so they can be previewed individually in the live preview app.

## Requirements

- Rust (edition 2024)

## Usage

Run the preview app:

```sh
cargo run
```

This launches a desktop app where you can interact with the components in a live preview.
