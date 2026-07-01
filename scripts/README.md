## Scripts

Utility scripts for project maintenance.

### rust_size_gate.py

Checks Rust source files for large files and large function bodies.

```bash
python3 scripts/rust_size_gate.py \
  --root . \
  --glob 'crates/**/*.rs' \
  --warn-file-lines 600 \
  --max-file-lines 800 \
  --warn-fn-lines 80 \
  --max-fn-lines 150
```

The function-size check is approximate. It is intended to catch oversized
functions early, not to replace code review.

### update-readme-version.sh

Updates version snippets in README files.

```bash
./scripts/update-readme-version.sh <new-version>
```

- Updates README version references.
- Keeps documentation snippets aligned with releases.
- Requires `python3`; override with `PYTHON_BIN` if needed.
