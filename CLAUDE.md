# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A CLI tool being built to calculate who gets a state-funded ("budgetary") scholarship place for master's
programs in Ukraine's 2026 admissions cycle. It scrapes institutions, offers, applications and applicants from
EDBO's public system (`vstup.edbo.gov.ua` / `registry.edbo.gov.ua`), decrypts the obfuscated applicant names, and
persists everything to Postgres. There is no placement/allocation algorithm here yet — parsing and normalization
only; that's a known future phase. Solo hobby project (see CONTRIBUTING.md), favor clarity and minimal
dependencies over cleverness.

**Currently mid-rewrite.** See `REFACTOR_PLAN.md` at the repo root for the live plan, current status, and the
reasoning behind it — check it before assuming any given module is wired up or planning further changes to
`engine`. The rest of this file describes the stable parts (`server`) and the target architecture; the
"Workspace layout" section below explains exactly what's live vs. orphaned in `engine` right now.

## Workspace layout (important gotcha)

The root `Cargo.toml` workspace has `members = ["server", "engine"]` — both are real workspace members, so plain
`cargo build`/`cargo test`/`cargo clippy` at the repo root cover both crates. (`engine` was previously a separate
crate named `edbo_core`, developed outside the workspace; it has since been renamed and added as a member.)

**The actual gotcha now is inside `engine` itself.** It's mid-rewrite: `engine/src/lib.rs` currently only
declares `mod config; mod database; mod errors; mod scraper;` — a small new skeleton (DB init, a config struct, an
error enum, a `Scraper` orchestrator whose `process()` is mostly a commented-out sketch). Every other `.rs` file
under `engine/src/` (`api/`, `context.rs`, `crypto.rs`, `dto/`, `model/`, `repository/`, `request.rs`,
`services/`) is the previous, fully-working implementation, still present on disk but **not** declared via `mod`
anywhere reachable from `lib.rs` — so it doesn't affect compilation, but it also isn't doing anything. Don't
assume any of those modules run just because the files exist; check `lib.rs`'s module list first, and see
`REFACTOR_PLAN.md` for the plan to re-wire them.

```bash
cargo build                                   # covers both server and engine
cargo test --package engine                   # currently trivial — most code isn't wired in yet
cargo test --package engine crypto::tests     # only works once `crypto` is `mod`-declared in lib.rs
cargo clippy --workspace --all-targets

cargo build --package server --release        # what the Dockerfile runs
cargo run --package server

# Formatting (whole repo, uses rustfmt.toml: max_width 90, edition 2024)
cargo fmt --all
```

`server` currently only loads config, validates runtime settings, and sets up logging (`server/src/main.rs`) —
it does not yet depend on or call into `engine`. `REFACTOR_PLAN.md` covers the concrete wiring (add `engine` as a
path dependency, construct `EngineConfig` from `RuntimeSettings.database_url`, call `Scraper::new(...).process()`).

## Toolchain & lints

- Pinned via `rust-toolchain.toml`: Rust `1.97.1`, edition 2024, with `rustfmt`/`clippy` components.
- Workspace-wide clippy/rust lints (`Cargo.toml` `[workspace.lints]`) **deny** `unwrap_used`, `expect_used`,
  `panic`, `indexing_slicing`, and `unsafe_code`, and **warn** on `clippy::pedantic` + `clippy::nursery` (with a
  few pedantic lints explicitly allowed back: `inconsistent_struct_constructor`, `missing_errors_doc`,
  `must_use_candidate`). Both crates opt in via `[lints] workspace = true`. This means:
  - No `.unwrap()`/`.expect()`/`panic!()` — use `Result`/`thiserror` error enums and `?`, or `.ok_or(...)`.
  - No direct slice/array indexing (`v[i]`) — use `.get(i)`/`.get_mut(i)` with explicit error handling (see
    `engine/src/crypto.rs` for the pattern used with fixed-size key/IV buffers).
  - Pedantic lints like `cast_possible_truncation` and `uninlined_format_args` are real signal here, not noise —
    the DB-schema/model type-width mismatches tracked in `REFACTOR_PLAN.md` were partly surfaced this way.
- CONTRIBUTING.md: only stable Rust features, one logical change per PR/commit, format with rustfmt.

## `engine` architecture (target state — see `REFACTOR_PLAN.md` for current re-wiring progress)

Entry point is `Scraper::process()` (`engine/src/scraper.rs`), which — once fully re-wired — runs a fixed
pipeline against a `Database`: enums → institutions → offers-with-institutions → offers →
applications/applicants, collecting everything into a `Context` (`engine/src/context.rs`).

**Service → Repository pattern**, one pair per entity (`services/*.rs` + `repository/*.rs`, e.g. `offer`,
`institution`, `applications`, `offer_university`, plus lookup-table services/repos for `degree`, `region`,
`study_form`, `offer_type`, `ownership_form`, `institution_category`, `speciality`, `knowledge_field`, `status`).
Every service follows the same idempotent-import shape (see `services/offer.rs`):

1. Ask its `Repository::is_empty()`.
2. If empty: fetch from the EDBO HTTP API (`api/*.rs`), insert each record via the repository, return the fetched list.
3. If not empty: read straight back from Postgres via `find_all()` and skip the network entirely.

This means the DB doubles as a cache — re-running the pipeline against a populated database does no HTTP calls.
To force a re-fetch, truncate the relevant tables.

Other pieces:
- `api.rs` / `api/*.rs`: HTTP client layer against EDBO endpoints. Requests are throttled by
  `INTERVAL_FOR_REQUESTS` (2s) and detect EDBO's own Ukrainian-language rate-limit error message
  (`ErrorResponse::handle_request_limit`), sleeping 60s when hit. **Not yet re-validated against the 2026 site** —
  EDBO appears to have been rebuilt on Next.js since these were written (see `REFACTOR_PLAN.md`'s de-risking
  research); the old registry JSON endpoint currently 404s. A live recon check found no Cloudflare
  challenge on the read paths tested, so the likely issue is moved/restructured endpoints rather than
  bot-protection, but this hasn't been confirmed end-to-end yet.
- `crypto.rs`: EDBO obfuscates applicant names in its public listings; this reimplements the site's
  AES-256-CBC (key/IV derived from an app number, `prsid`, and a hardcoded salt) decryption to recover them.
  Tests here are golden-value regression tests against real decrypted EDBO records — keep them passing. This
  formula has been independently cross-validated against a similar third-party project's reverse-engineering
  (see `REFACTOR_PLAN.md`) — high confidence it's correct and needs no changes.
- `database.rs`: `Database::init` uses `sqlx::migrate::MigrateDatabase` (`Postgres::database_exists`/
  `create_database`) to create the target database if needed, without requiring a separate admin-DB
  connection, then opens the real pool and runs embedded `sqlx::migrate!()` migrations (`engine/migrations/`).
- `model/*.rs`: DB-shaped structs, matching `engine/migrations/001_setup.sql` / `dbml/core_schema.dbml` (keep
  these three in sync when changing schema — `REFACTOR_PLAN.md` tracks known drift between them as of the
  current rewrite). `dto/*.rs` are the EDBO API response shapes — distinct from `model/*.rs`, converted on the
  way in.
- Errors are per-module enums (`ApiError`, `DbError`, `RepositoryError`, `CryptoError`, `ModelError`) composed
  via `thiserror`'s `#[from]` into the crate-level `EngineError` (`errors.rs`). `server` mirrors this with its
  own `ServerError` composing `ConfigError`/`LogsError`/`RuntimeSettingsError`.

**Hard rule:** never use `vstup.osvita.ua` (or any non-EDBO source) as a data source for this project, even as a
fallback if EDBO scraping proves difficult. EDBO only — this is an explicit decision by the project owner, not a
default to reconsider.

## `server` crate

Boot sequence in `server/src/main.rs`: load `Config` from `config.toml` next to the running executable
(`server/src/config.rs` — auto-creates a default template and returns an error on first run if missing, so the
operator can fill it in) → validate into `RuntimeSettings` (`server/src/settings.rs`, requires non-empty
`database_url`) → initialize the `fern`-based `Logger` (`server/src/logs.rs`, stdout or a directory of dated
log files). The root `config.toml` in this repo is a dev sample (gitignored per `.gitignore`, tracked copy here
is just a template) — real deployments mount their own via `docker-compose.prod.yaml`. `server` does not yet
depend on `engine` — see `REFACTOR_PLAN.md` for the wiring plan.

## Docker / local Postgres

- `docker-compose.dev.yaml`: Postgres only, for local development (`docker compose -f docker-compose.dev.yaml up -d`).
- `docker-compose.prod.yaml`: Postgres + the `server` app image, mounting `./config.toml` into the container.
- `Dockerfile`: multi-stage build, only ever builds `--package server --release` (engine is a library, not
  a binary target).
