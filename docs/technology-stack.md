# Technology Stack

The template targets mature Rust projects. Add runtime dependencies only when
the project needs them, but keep the verification toolchain strong by default.

## Default Policy

- Required path: Rust, Cargo, `cargo-nextest`, `cargo-deny`, and crates in this
  workspace.
- Optional convenience tools: `just`, `prek`, and release helpers.
- Project tasks belong in `crates/xtask`, not ad hoc scripts in another
  language.

## Recommended Crates

| Area | Recommendation |
|---|---|
| CLI | `clap` |
| Serialization | `serde`, `serde_json`, `toml` |
| Library errors | `thiserror` |
| Binary/xtask errors | `anyhow` |
| Observability | `tracing`, `tracing-subscriber` |
| Async runtime | `tokio` |
| HTTP client | `reqwest` |
| HTTP server | `axum` |
| Native desktop UI | `slint` |
| Web full-stack UI | `leptos` |
| Cross-platform UI | `dioxus` |
| gRPC | `tonic` |
| SQL | `sqlx` |
| ORM | Prefer no ORM; use `sea-orm` only when the project needs an entity model. |
| Parameterized tests | `rstest` |
| Property tests | `proptest` |
| Snapshot tests | `insta` |
| Trait mocks | `mockall` |
| HTTP mocks | `wiremock` |
| Benchmarks | `criterion` |
| TUI | `ratatui`, `crossterm` |

## Default Application Stacks

Use one of these stacks when evolving this template. Do not add all four stacks
to the shared base workspace.

| Variant | Default stack |
|---|---|
| Backend service | Rust 2024, `tokio`, `axum`, `tower`, `tower-http`, `reqwest`, `serde`, `thiserror`, `anyhow`, `tracing`, `tracing-subscriber`, `opentelemetry`, `opentelemetry-otlp`, `sqlx`, and PostgreSQL |
| Native desktop application | Rust 2024, `slint`, `tokio`, `cpal`, `reqwest`, `serde`, `thiserror`, `anyhow`, `tracing`, SQLite with `rusqlite`, Cargo, and `xtask` |
| Web full-stack application | Rust 2024, `leptos`, `tokio`, `axum`, `tower`, `tower-http`, `reqwest`, `serde`, `thiserror`, `anyhow`, `tracing`, `sqlx`, PostgreSQL, Cargo, and `cargo-leptos` |
| Cross-platform full-stack application | Rust 2024, `dioxus`, `tokio`, `axum`, `reqwest`, `serde`, `thiserror`, `anyhow`, `tracing`, `sqlx`, PostgreSQL, optional SQLite with `rusqlite`, Cargo, `dx`, and `xtask` |

## Server Stack

Choose this for APIs, workers with HTTP control planes, and backend services.

- Runtime: `tokio`.
- HTTP: `axum` with `tower`/`tower-http` middleware.
- Data: PostgreSQL with `sqlx`.
- Observability: `tracing`, `tracing-subscriber`, OpenTelemetry traces through
  OTLP.
- Errors: `thiserror` in library crates, `anyhow` at binary and `xtask`
  boundaries.
- Configuration: environment variables parsed at startup into typed config.
- Migrations: `sqlx migrate`.

Keep `domain` free of `tokio`, HTTP, SQL, filesystem, and process concerns.
Put request handlers, runtime wiring, and shutdown behavior at the binary
boundary. Tokio and Axum are defaults selected for the expected I/O-heavy
workload; they are not domain dependencies and may be replaced when measured
requirements justify a different runtime or HTTP framework.

## Desktop Stack

Choose this for the installed macOS and Windows application.

- Language and edition: Rust 2024.
- UI: `slint`; `.slint` declaration files are part of the native Rust UI build.
- Async runtime: `tokio` for background tasks. Keep the Slint event loop on the
  UI thread and send results back through explicit messages.
- Audio capture: `cpal`. Audio callbacks must only enqueue frames and must not
  perform network or database work.
- HTTP client: `reqwest` for ASR, LLM, and backend requests.
- Local persistence: SQLite through `rusqlite`, owned by a dedicated adapter or
  worker rather than the audio callback.
- Errors and diagnostics: `thiserror` in libraries, `anyhow` at the desktop
  entrypoint, and `tracing` for runtime diagnostics.
- Build and packaging: Cargo plus `xtask`, with platform-specific signing,
  notarization, installers, and microphone permission metadata documented on
  the desktop branch.

Do not add Tauri, React, TypeScript, Vite, Node.js, or a WebView. Keep Slint,
Tokio, cpal, reqwest, and rusqlite types out of `domain`; put concrete adapters
in `infra` and UI/runtime wiring in a desktop entrypoint crate.

## Web Full-Stack Stack

Choose this for a browser-first product.

- UI, routing, SSR, and hydration: `leptos`.
- HTTP and runtime: `axum` on `tokio`, with `tower`/`tower-http` middleware.
- Data: PostgreSQL with `sqlx` and `sqlx migrate`.
- Build: Cargo, `cargo-leptos`, and project automation in `xtask`.
- Styling: standards-based CSS without a required Node.js toolchain.
- Boundaries: keep Leptos components in a web crate and Axum wiring in a server
  crate. Share DTOs only when both sides genuinely use the same contract.

The browser output includes HTML, CSS, WebAssembly, and generated browser glue,
but application code and build automation remain in Rust. Do not require React,
TypeScript, Vite, or Node.js.

## Cross-Platform Full-Stack Stack

Choose this when one component model should cover web, Windows, macOS, Linux,
iOS, and Android.

- Shared UI, routing, SSR, hydration, and server functions: `dioxus`.
- Runtime and hosted HTTP boundary: `tokio` and `axum`.
- Remote data: PostgreSQL with `sqlx`; local client data may use SQLite through
  `rusqlite` when offline behavior requires it.
- Client HTTP: `reqwest` behind an app port when direct requests are needed.
- Build: Cargo, the Dioxus CLI (`dx`), and project automation in `xtask`.
- Platform capabilities: define ports for storage, microphone, notifications,
  deep links, and secure credentials, then implement platform adapters.

Dioxus maximizes UI reuse but uses platform WebViews for its conventional
desktop targets. Mobile builds also require Xcode or the Android SDK/NDK. Pick
the native desktop variant instead when avoiding WebViews is a hard constraint.
Do not promise complete code reuse: platform permissions, packaging, signing,
and device integrations remain target-specific.

## Backend Framework Policy

Use `axum` for backend HTTP services. It fits the Tokio/Tower ecosystem, keeps
handlers explicit, and composes with `tower` middleware.

Do not add an HTTP server framework to the base template. Add `axum` only to a
variant that hosts HTTP services. Native clients may use `reqwest` without
depending on the backend implementation.
