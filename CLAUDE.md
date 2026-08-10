# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A parser for admissions data exported from the Ukrainian EDBO (`vstup.edbo.gov.ua` / `registry.edbo.gov.ua`)
admissions system. It scrapes institutions, offers, applications and applicants from EDBO's public API,
decrypts the obfuscated applicant names, and persists everything to Postgres. There is no placement/allocation
algorithm here — parsing and normalization only. Solo hobby project (see CONTRIBUTING.md), favor clarity and
minimal dependencies over cleverness.

## Workspace layout (important gotcha)

The root `Cargo.toml` workspace currently has `members = ["server"]` only. **`edbo_core` is a separate crate
that is NOT part of the workspace** — it's mid-refactor and not yet wired up as a dependency of `server`. Do not
assume `cargo build`/`cargo test` at the repo root touches `edbo_core`; you must target it explicitly:

```bash
# edbo_core (the scraper/parser/DB library) — must target its manifest directly
cargo build --manifest-path edbo_core/Cargo.toml
cargo test --manifest-path edbo_core/Cargo.toml
cargo test --manifest-path edbo_core/Cargo.toml crypto::tests::test1   # single test
cargo clippy --manifest-path edbo_core/Cargo.toml --all-targets

# server (the binary, currently just boots config/logging) — workspace member
cargo build
cargo build --package server --release   # what the Dockerfile runs
cargo run --package server

# Formatting (whole repo, uses rustfmt.toml: max_width 90, edition 2024)
cargo fmt --all
```

`server` currently only loads config, validates runtime settings, and sets up logging (`server/src/main.rs`) —
it does not yet call into `edbo_core::process`. This reflects the recent history: "Refactor: Deleted redundant
common library, fully rewritten CLI app" → "Refactor: Cli -> Server", i.e. the old CLI/common-lib setup was torn
out and `server` is being rebuilt from scratch. Expect to eventually wire `server` to depend on and invoke
`edbo_core`.

## Toolchain & lints

- Pinned via `rust-toolchain.toml`: Rust `1.97.1`, edition 2024, with `rustfmt`/`clippy` components.
- Workspace-wide clippy/rust lints (`Cargo.toml` `[workspace.lints]`) **deny** `unwrap_used`, `expect_used`,
  `panic`, `indexing_slicing`, and `unsafe_code`. Both crates opt in via `[lints] workspace = true`. This means:
  - No `.unwrap()`/`.expect()`/`panic!()` — use `Result`/`thiserror` error enums and `?`, or `.ok_or(...)`.
  - No direct slice/array indexing (`v[i]`) — use `.get(i)`/`.get_mut(i)` with explicit error handling (see
    `edbo_core/src/crypto.rs` for the pattern used with fixed-size key/IV buffers).
- CONTRIBUTING.md: only stable Rust features, one logical change per PR/commit, format with rustfmt.

## `edbo_core` architecture

Entry point is `edbo_core::process()` in `edbo_core/src/lib.rs`, which runs a fixed pipeline against a
`Database`: enums → institutions → offers-with-institutions → offers → applications/applicants, collecting
everything into a `Context` (`edbo_core/src/context.rs`).

**Service → Repository pattern**, one pair per entity (`services/*.rs` + `repository/*.rs`, e.g. `offer`,
`institution`, `applications`, `offer_university`, plus lookup-table services/repos for `degree`, `region`,
`study_form`, `offer_type`, `ownership_form`, `institution_category`, `speciality`, `knowledge_field`, `status`).
Every service follows the same idempotent-import shape (see `services/offer.rs`):

1. Ask its `Repository::is_empty()`.
2. If empty: fetch from the EDBO HTTP API (`api/*.rs`), insert each record via the repository, return the fetched list.
3. If not empty: read straight back from Postgres via `find_all()` and skip the network entirely.

This means the DB doubles as a cache — re-running `process()` against a populated database does no HTTP calls.
To force a re-fetch, truncate the relevant tables.

Other pieces:
- `api.rs` / `api/*.rs`: HTTP client layer against EDBO endpoints. Requests are throttled by
  `INTERVAL_FOR_REQUESTS` (2s) and detect EDBO's own Ukrainian-language rate-limit error message
  (`ErrorResponse::handle_request_limit`), sleeping 60s when hit.
- `crypto.rs`: EDBO obfuscates applicant names in its public listings; this reimplements the site's
  AES-256-CBC (key/IV derived from an app number, `prsid`, and a hardcoded salt) decryption to recover them.
  Tests here are golden-value regression tests against real decrypted EDBO records — keep them passing.
- `database.rs`: `Database::init` connects to the `postgres` admin DB first to create the target database if
  it doesn't exist yet, then opens the real pool and runs embedded `sqlx::migrate!()` migrations
  (`edbo_core/migrations/`).
- `model/*.rs`: DB-shaped structs, matching `edbo_core/migrations/001_setup.sql` / `dbml/core_schema.dbml`
  (keep these three in sync when changing schema). `dto/*.rs` are the EDBO API response shapes — distinct
  from `model/*.rs`, converted on the way in.
- Errors are per-module enums (`ApiError`, `DbError`, `RepositoryError`, `CryptoError`, `ModelError`) composed
  via `thiserror`'s `#[from]` into the crate-level `CoreError` (`error.rs`). `server` mirrors this with its own
  `ServerError` composing `ConfigError`/`LogsError`/`RuntimeSettingsError`.

## `server` crate

Boot sequence in `server/src/main.rs`: load `Config` from `config.toml` next to the running executable
(`server/src/config.rs` — auto-creates a default template and returns an error on first run if missing, so the
operator can fill it in) → validate into `RuntimeSettings` (`server/src/settings.rs`, requires non-empty
`database_url`) → initialize the `fern`-based `Logger` (`server/src/logs.rs`, stdout or a directory of dated
log files). The root `config.toml` in this repo is a dev sample (gitignored per `.gitignore`, tracked copy here
is just a template) — real deployments mount their own via `docker-compose.prod.yaml`.

## Docker / local Postgres

- `docker-compose.dev.yaml`: Postgres only, for local development (`docker compose -f docker-compose.dev.yaml up -d`).
- `docker-compose.prod.yaml`: Postgres + the `server` app image, mounting `./config.toml` into the container.
- `Dockerfile`: multi-stage build, only ever builds `--package server --release` (edbo_core is a library, not
  a binary target).