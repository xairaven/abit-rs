# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A CLI tool being built to calculate who gets a state-funded ("budgetary") scholarship place for master's
programs in Ukraine's 2026 admissions cycle. It scrapes institutions, offers, applications and applicants from
EDBO's public system (`vstup.edbo.gov.ua` / `registry.edbo.gov.ua`), decrypts the obfuscated applicant names,
persists everything to Postgres, and — once the algorithm is built — computes and serves who actually gets a
budgetary place. Solo hobby project (see CONTRIBUTING.md), favor clarity and minimal dependencies over
cleverness.

**Currently mid-rewrite.** See `REFACTOR_PLAN.md` at the repo root for the live plan, current status, and the
reasoning behind every architectural decision below — check it before assuming any given crate/module is wired
up or planning further changes.

## Workspace layout

Root `Cargo.toml` members: `["server", "scraper", "model"]`. A fourth crate, `placement` (the allocation
algorithm), is planned but not yet created.

- **`server`**: the CLI entry point and orchestrator. `server/src/main.rs` boots `Config` → `RuntimeSettings` →
  `Logger`, then `server/src/database.rs`'s `Database::init` ensures the target Postgres database exists
  (`Postgres::database_exists`/`create_database`, no admin-DB connection needed) and opens the pool (bound to a
  `let db = ...`, kept alive). **Not yet wired further**: `main.rs` builds the pool but doesn't yet construct
  `scraper::Scraper` or call `.process()` — that's the next concrete step. `server/Cargo.toml` depends on
  `scraper` and on `sqlx` directly.
- **`scraper`** (renamed `edbo_core` → `engine` → `scraper`): data acquisition + persistence against EDBO.
  Organized **by entity, DDD-style** — everything about one domain concept lives together. `institution.rs`
  (just `pub mod api; pub mod dto; pub mod errors; pub mod service;`) + `institution/`:
  - `dto.rs` — `InstitutionDto` (raw EDBO JSON shape, serde `rename` per Ukrainian column header) +
    `impl TryFrom<InstitutionDto> for Institution` (legal despite `Institution` living in the separate `model`
    crate — see the orphan-rule note in `REFACTOR_PLAN.md`).
  - `api.rs` — `InstitutionApi`, an associated-function-only unit struct wrapping the HTTP fetch
    (`reqwest::get` → `serde_json::from_str::<Vec<InstitutionDto>>`).
  - `service.rs` — `InstitutionService<'a>` (holds `&'a Database`), the fetch-or-cache orchestrator:
    `get()` checks `is_empty()`, either fetches+persists or reads back via `find_all()`.
  - `errors.rs` — `InstitutionError`, one variant per failure point (DTO parse, request, SQL op, inconsistent
    dictionary data on read-back), composed into the crate-level `ScraperError` via `#[from]`.

  This struct-with-`impl`-block shape (not free functions) is a **deliberate, explicit project convention** —
  the project owner prefers methods on a struct over bare functions taking the same parameter every call, for
  clarity/conciseness at call sites. Follow this shape for `offer`/`application`/`applicant` once they're built:
  one `<entity>.rs` + `<entity>/{api,dto,errors,service}.rs` per entity, same 4-piece template.
  `Scraper` (`lib.rs`) takes a `&PgPool` (no config struct — `server` owns the connection string entirely) and
  `Scraper::process()` currently calls `Database::configure(&pool)` (checks `common`/`scraped` schemas+tables
  exist, runs `sqlx::migrate!()` if not) then `InstitutionService::new(&self.database).get()`. Depends on `model`.
- **`model`**: shared domain types, kept dependency-light (`num_enum`, `strum`) so a future `placement` crate
  can depend on just types, not `scraper`'s reqwest/sqlx. `Institution` (title, id, parent_id, short_name,
  english_name, is_from_crimea, registration_date, category, ownership_form, region — all in
  `model/src/institution.rs`), with `InstitutionCategory` (`institution/category.rs`) and `OwnershipForm`
  (`institution/ownership.rs`) nested under `pub mod institution;` — by-ownership grouping, not flat
  one-file-per-enum. `Region` lookup enum at crate root (`region.rs`). All three lookup enums are
  `#[repr(i16)]` + `Copy, Clone` + `strum::EnumString`/`Display` with `#[strum(serialize = "...")]` mapping
  EDBO's Ukrainian text labels directly to variants — matches their `common.*` `SMALLINT` columns exactly, no
  casts needed at the sqlx boundary. `schemas.rs` holds flat `COMMON`/`SCRAPED`/`PLACEMENT` schema-name
  constants shared across crates.
- **`placement`** (the allocation algorithm) — **not created yet**, planned.
- **`edbo_core`** at the repo root: a complete, standalone reference copy of last year's fully-working
  implementation. **Deliberately not a workspace member** — don't add it, don't try to build it. It exists
  purely to consult and port logic from while designing `scraper`/`model` fresh. See `REFACTOR_PLAN.md` for the
  full inventory of what's reusable as-is vs. needs rework.

```bash
cargo build                                   # covers server + scraper + model
cargo test --package scraper
cargo clippy --workspace --all-targets

cargo build --package server --release        # what the Dockerfile runs
cargo run --package server

# Formatting (whole repo, uses rustfmt.toml: max_width 90, edition 2024)
cargo fmt --all
```

## Toolchain & lints

- Pinned via `rust-toolchain.toml`: Rust `1.97.1`, edition 2024, with `rustfmt`/`clippy` components.
- Workspace-wide clippy/rust lints (`Cargo.toml` `[workspace.lints]`) **deny** `unwrap_used`, `expect_used`,
  `panic`, `indexing_slicing`, and `unsafe_code`, and **warn** on `clippy::pedantic` + `clippy::nursery` (with a
  few pedantic lints explicitly allowed back: `inconsistent_struct_constructor`, `missing_errors_doc`,
  `must_use_candidate`). All crates opt in via `[lints] workspace = true`. This means:
  - No `.unwrap()`/`.expect()`/`panic!()` — use `Result`/`thiserror` error enums and `?`, or `.ok_or(...)`.
  - No direct slice/array indexing (`v[i]`) — use `.get(i)`/`.get_mut(i)` with explicit error handling.
  - `scraper` currently builds and clippy-checks (`--all-targets`, pedantic+nursery included) with **zero
    warnings** against the live schema — keep it that way as more entities land.

## Architecture (see `REFACTOR_PLAN.md` for full rationale)

**No `Service`/`Repository` *traits*.** `edbo_core`'s old pattern (a `Service` trait wrapping a `Repository`
trait, one impl of each per entity, plus an `EnumService` god-struct seeding 9 lookup tables via 9 copy-pasted
blocks) was never used polymorphically anywhere — pure ceremony with no payoff. The replacement is **plain
structs with `impl` blocks, no traits, grouped by entity** (see `institution/service.rs` as the reference
example) — the project owner deliberately prefers methods-on-a-struct over bare functions for call-site clarity,
so don't push back toward free functions; the actual fix that mattered was dropping the *trait* (zero
polymorphic dispatch anywhere) and the cross-entity god-struct (`EnumService`), not the struct shape itself.

**`dto` vs `model` — keep this split, it earns its keep.** `InstitutionDto` (in `scraper`) and `Institution` (in
`model`) genuinely diverge (EDBO's raw Ukrainian-labeled wire format vs. validated domain data) — unlike
`Service`/`Repository`, this isn't cargo-culted. The conversion is a normal `impl TryFrom<InstitutionDto> for
Institution` trait impl living in `scraper` — legal under Rust's orphan rule because `InstitutionDto` (local to
`scraper`) appears as `TryFrom`'s generic parameter, even though `Institution` (the `Self` type) lives in the
foreign `model` crate. (The orphan rule only requires *some* type in the impl header to be local — it doesn't
have to be `Self`.)

**Crate topology:** `model` (plain domain types, minimal deps) ← `scraper` (fetches from EDBO, persists —
already depends on `model`) and `placement` (pure allocation algorithm, reads `model` types) both depend on
`model`; `server` orchestrates `scraper` + `placement`.

**Database: one Postgres database, three schemas**, already reflected in `scraper/migrations/001_setup.sql` —
`common` (reference/lookup tables + `offer`/`offers_institutions` — anything that isn't a real person's data,
written by `scraper`), `scraped` (`applicant`/`application` — the actual PII-bearing scraped input, written by
`scraper`), `placement` (algorithm output, not created yet). Not three databases — Postgres can't join across
databases without extensions, but schemas join/FK freely within one DB. Schema-level separation also gives a
real permission boundary later — a role serving results publicly can be granted access to `placement`+`common`
only, never `scraped`'s raw PII.

## EDBO data-source status (see `REFACTOR_PLAN.md` for full detail)

**Institutions: solved and implemented.** Real endpoint (confirmed live, both `.xlsx` and `.json` export
formats work — `scraper` uses `exp=json`, no spreadsheet-parsing dependency needed after all):
```
GET https://registry.edbo.gov.ua/api/opendata/universities?rg=<region>&ut=<category>&exp=json
```
`ut` is `InstitutionCategory`'s numeric code, `rg` is `Region`'s — confirmed by the fact the exported data's
category text matched exactly one `InstitutionCategory` variant when `ut=1` was used. `scraper` hardcodes
`ut=1` (`HigherEducation`) and `rg=0` (`Region::Every`, fetches every region in one call) since master's
programs are only offered by category-1 institutions — not looping over the other 7 categories.

**Offers (and presumably applications): a real, confirmed, much harder obstacle — Cloudflare Turnstile + genuine
hybrid encryption.** `vstup.edbo.gov.ua/offers` gates its data behind a Turnstile challenge (auto-solves in a
real browser session) and a Next.js Server-Actions API (not REST — POSTs to the page URL itself with a
`next-action` content-hash header, fragile across deploys) whose payloads are wrapped in a real, per-session
RSA-OAEP + AES-GCM hybrid handshake (both directions: client encrypts requests with the server's public key,
server encrypts responses with an ephemeral public key the client generates per session). This is **not** the
old system's fixed/computable obfuscation — there's no static secret to extract from the JS bundle. **Decision:
drive a real headless browser** (`chromiumoxide` recommended) and scrape the rendered DOM after the real EDBO
JS does its own decryption, rather than reimplementing the crypto. See `REFACTOR_PLAN.md` for the concrete
approach and caveats (Docker image impact, session reuse, name collision with the `scraper` HTML-parsing crate).

**Master's admission has no quota categories** (unlike bachelor's — confirmed by the project owner). Ranking is
a nationwide per-speciality competitive list ("широкий конкурс"), one main admission wave, tie-broken by grade
components — see `REFACTOR_PLAN.md` for the full algorithm research.

**Hard rule:** never use `vstup.osvita.ua` (or any non-EDBO source) as a data source for this project, even as a
fallback if EDBO scraping proves difficult. EDBO only — explicit project-owner decision, not a default to
reconsider.

## Docker / local Postgres

- `docker-compose.dev.yaml`: Postgres only, for local development (`docker compose -f docker-compose.dev.yaml up -d`).
- `docker-compose.prod.yaml`: Postgres + the `server` app image, mounting `./config.toml` into the container.
- `Dockerfile`: multi-stage build, only ever builds `--package server --release`. **Will need real changes**
  once headless-browser-based offer scraping lands — the current `debian:bookworm-slim` base has none of
  Chrome/Chromium's system dependencies.