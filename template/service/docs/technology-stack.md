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
| Observability | `tracing`, `tracing-subscriber`, optional OpenTelemetry traces |
| Async runtime | `tokio` |
| HTTP client | `reqwest` |
| HTTP server | `axum` |
| gRPC | `tonic` |
| SQL | `sqlx` |
| ORM | `sea-orm` or `diesel` |
| Parameterized tests | `rstest` |
| Property tests | `proptest` |
| Snapshot tests | `insta` |
| Trait mocks | `mockall` |
| HTTP mocks | `wiremock` |
| Benchmarks | `criterion` |

## Web Framework Guidance

Prefer `axum` for new HTTP services. It fits the Tokio/Tower ecosystem, keeps
handlers explicit, and composes well with middleware.

Use `actix-web` when the team specifically wants Actix's model or has a strong
performance/operational reason.

Use `rocket` when a project values a more batteries-included web framework and
the tradeoff is intentional.

Do not add a web framework to the base template. Add it only to a project or a
separate template variant that actually serves HTTP.
