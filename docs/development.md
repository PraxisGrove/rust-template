# Development

The base workflow uses Cargo plus `cargo-nextest` and `cargo-deny`. Optional
tools can improve local ergonomics, but the mature-project gate should stay
explicit and reproducible.

## Required Gates

Run these before handing off a change:

```bash
cargo fmt --all --check
cargo check --workspace --all-targets
cargo nextest run --workspace --all-targets
cargo test --workspace --doc
cargo clippy --workspace --all-targets -- -D warnings -A clippy::too_many_lines
cargo clippy --workspace --all-targets -- -W clippy::too_many_lines
cargo deny check
cargo build --workspace --all-targets --release
cargo run -p xtask -- size
```

Use `cargo fmt --all` when you want to apply formatting.

## Optional Shortcuts

If `just` is installed, the `justfile` provides shortcuts for the same commands:

```bash
just ci
just test
just test-doc
just clippy
just deny
just size
```

Do not document `just` as a required setup step. CI should use Cargo directly.

## Dependency Changes

Declare shared versions in `[workspace.dependencies]` in the root `Cargo.toml`.
Member crates should use `workspace = true` where possible.

When adding a dependency, include the reason in the change description. Prefer
small, maintained crates with a clear API and avoid broad feature sets unless
the project needs them.

## Size Guidance

The xtask size command warns when Rust files exceed 600 lines. It reports file
length only and never fails because of size. Function length is checked by
Clippy's syntax-aware `too_many_lines` lint with a 150-line threshold.

Size warnings should trigger a review of cohesion and control flow, not an
automatic split. A split is useful only when it reduces the context needed to
understand and change the code.
