# Rust Template

A Rust workspace template optimized for human and AI-assisted development:
clear crate boundaries, standard Cargo gates, small public APIs, and explicit
contributor instructions.

This is not an AI application framework. It is a general Rust template designed
to keep projects easy for humans and coding agents to understand, modify,
verify, and review.

## Structure

```text
crates/
  domain/  # business types and rules
  app/     # use cases and ports
  infra/   # concrete implementations for app ports
  cli/     # binary entrypoint and dependency wiring
E2E/       # optional end-to-end tests
```

The intended dependency direction is:

```text
cli -> app -> domain
cli -> infra -> app
```

Keep `domain` free of infrastructure and entrypoint concerns. Put traits that
describe required capabilities near the use cases that consume them.

## Tooling Policy

The required development path uses the standard Rust toolchain only:

```bash
cargo fmt
cargo check
cargo test
cargo clippy
cargo build
```

Extra tools such as `just`, `uv`, `prek`, `cargo-nextest`, `cargo-deny`, or
release helpers are optional. They can improve local workflow, but this template
must stay usable without installing them.

## Development

Run the full local gate with Cargo:

```bash
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --release
python3 scripts/rust_size_gate.py --root . --glob 'crates/**/*.rs' --warn-file-lines 600 --max-file-lines 800 --warn-fn-lines 80 --max-fn-lines 150
```

When you want Cargo to format the code:

```bash
cargo fmt --all
```

Run the sample CLI:

```bash
cargo run -p template-cli -- workspace
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
engineering guidance lives under `docs/`:

- `docs/architecture.md`
- `docs/development.md`
- `docs/testing.md`
- `docs/review.md`

These files are part of the template contract. Keep them current when changing
crate layout, required gates, or review policy.

## Optional `just` Shortcuts

`just` is not required. If it is installed, the included `justfile` provides
short aliases for the same Cargo commands:

```bash
just ci
just test
just clippy
just size
just fmt-fix
```

CI and project documentation should continue to use Cargo commands directly so
new users do not need extra tools before the project builds.

## Tests

Keep fast Rust tests in the workspace:

```bash
cargo test --workspace --all-targets
```

End-to-end tests live under `E2E/`. They are optional for the base Rust workflow
and may use Python tooling such as `uv`, `pytest`, or `behave` when the project
needs them.
