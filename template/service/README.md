# {{project-name}}

{{project_description}}

This is a Rust backend service generated from the service profile. It uses
`tokio`, `axum`, `sqlx`, PostgreSQL readiness checks, `tracing`, and optional
OpenTelemetry traces on top of the shared mature Rust workspace gate.

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

Start the service:

```bash
cargo run -p {{project-name}}-cli
```

The default bind address is `127.0.0.1:3000`. Override it with
`APP_ADDR=0.0.0.0:3000`.

Set `DATABASE_URL` to enable PostgreSQL readiness checks:

```bash
DATABASE_URL=postgres://postgres:postgres@localhost:5432/{{project-name}} \
cargo run -p {{project-name}}-cli
```

When `DATABASE_URL` is empty or missing, `/health/ready` returns ready with a
`database readiness check skipped` detail so the generated service runs locally
without external infrastructure.

Set `OTEL_EXPORTER_OTLP_ENDPOINT` to enable OTLP trace export. `OTEL_SERVICE_NAME`
overrides the default service name.

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

## Database Migrations

SQL migrations live in `migrations/`. This profile provides the directory and
`just` shortcuts, but does not define application tables for you:

```bash
sqlx migrate add create_example
sqlx migrate run
sqlx migrate info
```
