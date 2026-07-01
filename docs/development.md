# Development

The base workflow uses the standard Rust toolchain. Optional tools can improve
local ergonomics, but the template must remain usable without them.

## Required Gates

Run these before handing off a change:

```bash
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --release
python3 scripts/rust_size_gate.py --root . --glob 'crates/**/*.rs' --warn-file-lines 600 --max-file-lines 800 --warn-fn-lines 80 --max-fn-lines 150
```

Use `cargo fmt --all` when you want to apply formatting.

## Optional Shortcuts

If `just` is installed, the `justfile` provides shortcuts for the same commands:

```bash
just ci
just test
just clippy
just size
```

Do not document `just` as a required setup step. CI should use Cargo and Python
directly.

## Dependency Changes

Declare shared versions in `[workspace.dependencies]` in the root `Cargo.toml`.
Member crates should use `workspace = true` where possible.

When adding a dependency, include the reason in the change description. Prefer
small, maintained crates with a clear API and avoid broad feature sets unless
the project needs them.

## Size Gate

The size gate is intentionally approximate. Its job is to catch large files and
large functions early, especially during AI-assisted development where changes
can grow quickly.

Warnings should trigger a split plan. Errors should block the change unless
there is a documented reason and a short-term migration plan.
