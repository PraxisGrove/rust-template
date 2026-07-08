# {{project-name}}

{{project_description}}

This is a mature Rust workspace with clear crate boundaries, required
dependency-policy checks, nextest-based test execution, small public APIs, and
explicit contributor instructions.

## Structure

```text
crates/
  domain/  # business types and rules
  app/     # use cases and ports
  infra/   # concrete implementations for app ports
  cli/     # binary entrypoint and dependency wiring
  xtask/   # Rust-only project maintenance tasks
```

The intended dependency direction is:

```text
cli -> app -> domain
cli -> infra -> app
```

Keep `domain` free of infrastructure and entrypoint concerns. Put traits that
describe required capabilities near the use cases that consume them.

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

When you want Cargo to format the code:

```bash
cargo fmt --all
```

Run the sample CLI:

```bash
cargo run -p {{project-name}}-cli -- workspace
```

## Workspace Conventions

Shared package metadata, dependency versions, lints, and build profiles live in
the root `Cargo.toml`. Crates should inherit them:

```toml
edition.workspace = true
license.workspace = true

[lints]
workspace = true
```

Prefer declaring third-party dependencies under `[workspace.dependencies]` and
using `workspace = true` from member crates. This keeps versions and feature
choices visible in one place.

## AI-Assisted Development

Project-level instructions for coding agents live in `AGENTS.md`. Human-facing
engineering guidance lives under `docs/`.

These files are part of the project contract. Keep them current when changing
crate layout, required gates, or review policy.

Production code should fail early with explicit errors and validated types, not
late with assertions or panics. Tests may still use assertions to verify
behavior.

## Optional `just` Shortcuts

`just` is not required. If it is installed, the included `justfile` provides
short aliases for the required gate commands:

```bash
just ci
just test
just test-doc
just clippy
just deny
just size
just fmt-fix
```
