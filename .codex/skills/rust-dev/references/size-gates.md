# Size Guidance

Size checks flag files and functions that deserve a cohesion and control-flow
review. They do not prove that code should be split.

## Defaults

- File warning: over 600 lines.
- Function warning: over 150 lines, reported by Clippy's syntax-aware
  `too_many_lines` lint.

## Command

```bash
cargo run -p xtask -- size
```

The xtask command reports file warnings and exits successfully. Configure the
function threshold in `clippy.toml`.

## Notes

Warnings should prompt a review. Split only when the result is more cohesive or
requires less context to understand; long state machines and similarly linear,
cohesive code may be clearer when kept together.
