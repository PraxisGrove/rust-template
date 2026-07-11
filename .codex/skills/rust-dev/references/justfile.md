# Optional justfile

`just` is useful as a command shortcut, but this template must not require it.
Cargo commands are the source of truth.

## Recommended Recipes

- `fmt`: `cargo fmt --all --check`
- `fmt-fix`: `cargo fmt --all`
- `check`: `cargo check --workspace --all-targets`
- `test`: `cargo test --workspace --all-targets`
- `clippy`: runs strict Clippy with `too_many_lines` allowed, then runs that
  lint separately as a warning.
- `build`: `cargo build --workspace --all-targets --release`
- `size`: `cargo run -p xtask -- size`
- `ci`: run all gates

Keep complex logic in scripts. The `justfile` should remain a thin wrapper.
