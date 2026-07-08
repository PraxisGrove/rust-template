# {{project-name}}

{{project_description}}

This is a Rust backend service generated from the service profile. It uses
`tokio`, `axum`, `sqlx`, PostgreSQL, and `tracing` on top of the shared mature
Rust workspace gate.

## Structure

```text
crates/
  domain/  # pure service domain types
  app/     # use cases and port traits
  infra/   # sqlx/PostgreSQL adapters
  cli/     # axum server wiring and process behavior
  xtask/   # Rust-only project maintenance tasks
```

The intended dependency direction is:

```text
cli -> app -> domain
cli -> infra -> app
```

## Run

Set a PostgreSQL connection string and start the service:

```bash
DATABASE_URL=postgres://postgres:postgres@localhost:5432/{{project-name}} \
cargo run -p {{project-name}}-cli
```

The default bind address is `127.0.0.1:3000`. Override it with
`BIND_ADDRESS=0.0.0.0:3000`.

Health endpoints:

```text
GET /health/live
GET /health/ready
```

## Development

Run the full local gate before handing off a change:

```bash
cargo fmt --all --check
cargo check --workspace --all-targets
cargo nextest run --workspace --all-targets
cargo test --workspace --doc
cargo clippy --workspace --all-targets -- -D warnings
cargo deny check
cargo build --workspace --all-targets --release
cargo run -p xtask -- size
```

Use `cargo fmt --all` to apply formatting.
