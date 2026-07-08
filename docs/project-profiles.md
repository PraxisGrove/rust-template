# Project Profiles

This repository uses one template repository with multiple `cargo-generate`
profiles. Shared engineering rules stay in the root docs; each profile adds only
the framework and dependencies required for that application type.

## Profiles

| Profile | Use when | Stack |
|---|---|---|
| `template/base` | Starting a general Rust workspace, library, CLI, or unknown project type. | Cargo workspace, `domain/app/infra/cli/xtask`, nextest, deny, size gate. |
| `template/service` | Starting a backend HTTP service. | Base rules plus `tokio`, `axum`, `sqlx`, PostgreSQL readiness checks, and `tracing`. |

## Commands

```bash
cargo generate --git <repo> template/base --name my-project
cargo generate --git <repo> template/service --name my-service
```

For local template development:

```bash
cargo generate --path . template/base --name my-project
cargo generate --path . template/service --name my-service
```

Verify all profiles:

```bash
cargo run -p xtask -- verify-profiles
```

Verify one profile:

```bash
cargo run -p xtask -- verify-profiles --profile template/service
```

## Policy

- Add shared engineering rules to root docs and mirror them into profiles when
  they affect generated projects.
- Add framework-specific rules only to the owning profile.
- Keep generated projects able to pass their documented gate immediately after
  generation.
- Add every new profile to `cargo run -p xtask -- verify-profiles` defaults.
- Do not force service dependencies such as `axum`, `sqlx`, or PostgreSQL into
  `template/base`.
