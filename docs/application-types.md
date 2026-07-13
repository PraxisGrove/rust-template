# Application Types

This repository is the shared base for four Rust-only variants. A project should
choose one variant and evolve the workspace in that direction. The variants
may live on separate Git branches, but their architecture and quality gates
start from the same base.

## Supported Directions

| Variant | Use when | Default stack |
|---|---|---|
| Backend service | The product needs an HTTP API, workers, shared remote state, or hosted integrations. | Rust 2024, `tokio`, `axum`, `tower`, `tower-http`, `serde`, `tracing`, OpenTelemetry, `sqlx`, and PostgreSQL. |
| Native desktop application | The product needs an installed macOS and Windows UI, local audio capture, local data, and native rendering without a WebView. | Rust 2024, `slint`, `tokio`, `cpal`, `reqwest`, SQLite with `rusqlite`, Cargo, and `xtask`. |
| Web full-stack application | The product is delivered through a browser and needs SSR, hydration, an HTTP API, and persistent server data. | Rust 2024, `leptos`, `tokio`, `axum`, `tower`, `serde`, `sqlx`, PostgreSQL, Cargo, and `cargo-leptos`. |
| Cross-platform full-stack application | The product prioritizes shared UI and application code across web, desktop, iOS, and Android and accepts platform WebViews where Dioxus uses them. | Rust 2024, `dioxus`, `tokio`, `axum`, `serde`, `reqwest`, `sqlx`, PostgreSQL, optional local SQLite with `rusqlite`, Cargo, `dx`, and `xtask`. |

## Selection Rules

- Pick the backend variant for hosted APIs and workers.
- Pick the native desktop variant when native rendering and avoiding WebViews
  matter more than sharing UI code with web and mobile targets.
- Pick the web full-stack variant for a browser-first product.
- Pick the cross-platform full-stack variant when maximizing shared UI code is
  more important than avoiding WebViews.
- When any client calls a backend, keep the HTTP contract explicit. Do not make
  the client depend on backend internals.
- Keep command-line tools as small entrypoints or `xtask` commands inside the
  selected variant rather than creating another template variant.

## Policy

- Keep the base template dependency-light.
- Use Rust and Cargo for application code, build automation, and packaging
  logic.
- The desktop variant may use `.slint` UI declaration files. It must not require
  Tauri, React, TypeScript, Vite, Node.js, or a WebView.
- The web and cross-platform variants may produce WebAssembly, HTML, CSS, and
  generated browser glue, but application and build automation remain in Rust.
- Mobile builds still require the platform SDKs mandated by Apple and Google.
- Do not add dependencies from all variants to the shared base. Add a
  framework only on the branch that needs it.
- Keep `domain`, `app`, `infra`, `cli`, and `xtask` boundaries unless there is
  a concrete reason to split or rename crates.
- Document application-specific choices in `docs/technology-stack.md` and
  `docs/architecture.md` when the project commits to them.
