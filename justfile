# Optional convenience commands.
#
# The template does not require `just`; every recipe below is a thin wrapper
# around commands documented in README.md.

set windows-shell := ["pwsh.exe", "-NoLogo", "-NoProfile", "-Command"]

default:
    @just --list

fmt:
    cargo fmt --all --check

fmt-fix:
    cargo fmt --all

check:
    cargo check --workspace --all-targets

test:
    cargo test --workspace --all-targets

clippy:
    cargo clippy --workspace --all-targets -- -D warnings

size:
    python3 scripts/rust_size_gate.py --root . --glob 'crates/**/*.rs' --warn-file-lines 600 --max-file-lines 800 --warn-fn-lines 80 --max-fn-lines 150

build:
    cargo build --workspace --all-targets --release

ci:
    cargo fmt --all --check
    cargo check --workspace --all-targets
    cargo test --workspace --all-targets
    cargo clippy --workspace --all-targets -- -D warnings
    cargo build --workspace --all-targets --release
    python3 scripts/rust_size_gate.py --root . --glob 'crates/**/*.rs' --warn-file-lines 600 --max-file-lines 800 --warn-fn-lines 80 --max-fn-lines 150

# Optional e2e helpers. Requires uv and the E2E project dependencies.
[unix]
e2e:
    cd E2E && uv run -- pytest -v --tb=short

[windows]
e2e:
    Set-Location E2E; uv run -- pytest -v --tb=short
