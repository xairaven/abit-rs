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

- **`scraper`** (renamed from `edbo_core`/`engine`): data acquisition + persistence against EDBO. Currently a
  genuinely blank skeleton — `scraper/src/` has only `config.rs`, `database.rs`, `errors.rs`, `scraper.rs`,
  `lib.rs`. The internal type names (`EngineConfig`, `EngineError`) still reflect the pre-rename crate name and
  should eventually become `ScraperConfig`/`ScraperError` for consistency — not urgent, just a known leftover.
- **`model`**: intended to hold shared domain types (lookup enums, `Institution`/`Offer`/`Application`/
  `Applicant`, and eventually `PlacementResult`) so a future `placement` crate can depend on just this — not on
  `scraper`'s `reqwest`/`sqlx`/crypto dependencies. Currently an empty scaffold (`model/src/lib.rs` has no
  content, `Cargo.toml` has no dependencies yet).
- **`edbo_core`** at the repo root: a complete, standalone reference copy of last year's fully-working
  implementation. **Deliberately not a workspace member** — don't add it, don't try to build it. It exists
  purely to consult and port logic from while designing `scraper`/`model` fresh. See `REFACTOR_PLAN.md` for the
  full inventory of what's reusable as-is vs. needs rework.
- **`server`**: the CLI entry point. Currently only boots config/settings/logging (`server/src/main.rs`) — does
  not yet depend on `scraper` or `model`.

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
  - Pedantic lints like `cast_possible_truncation` are real signal here, not noise — see the schema-drift
    fixes tracked in `REFACTOR_PLAN.md`.
- CONTRIBUTING.md: only stable Rust features, one logical change per PR/commit, format with rustfmt.

## Architecture (see `REFACTOR_PLAN.md` for full rationale)

**No `Service`/`Repository` traits.** `edbo_core`'s old pattern (a `Service` trait wrapping a `Repository`
trait, one impl of each per entity, plus an `EnumService` god-struct seeding 9 lookup tables via 9 copy-pasted
blocks) was never used polymorphically anywhere — pure ceremony with no payoff. The replacement:
- **Lookup tables** (`degree`, `region`, `study_form`, etc.): one `macro_rules!` template generating a plain,
  fully `sqlx::query!`-compile-time-checked seed function per table — no trait, no struct.
- **Real entities** (`institution`, `offer`, `application`, ...): one plain module per entity, `pub`
  orchestration function + private SQL-access functions in the same file — layering by visibility, not by
  trait/type boundary.

**`dto` vs `model` — keep this split, it earns its keep.** Every DTO type has exactly two consumers (the
fetcher that deserializes it, the converter that turns it into a domain type) and the shapes genuinely diverge
(EDBO's raw wire format vs. validated domain data) — unlike `Service`/`Repository`, this isn't cargo-culted.

**Crate topology (target, not fully built yet):** `model` (plain domain types, minimal deps: serde, thiserror,
strum, num_enum, bigdecimal — no reqwest/sqlx/crypto) ← `scraper` (fetches from EDBO, persists) and `placement`
(pure allocation algorithm, reads `model` types) both depend on `model`; `server` orchestrates `scraper` +
`placement`. One orphan-rule consequence of this split: `impl TryFrom<InstitutionDto> for Institution` can't
live as a trait impl once `Institution` moves to `model` and `InstitutionDto` stays in `scraper` — it becomes a
plain free function in `scraper` instead (e.g. `fn institution_from_dto(dto) -> Result<model::Institution, _>`).

**Database: one Postgres database, three schemas** (not three databases — Postgres can't join across
databases without extensions, but schemas join/FK freely within one DB): `common` (reference/lookup tables,
written by `scraper`), `scraped` (EDBO input data — offer/application/applicant, written by `scraper`),
`placement` (algorithm output, written by the future `placement` crate, read by `server` to serve to users).
Schema-level separation also gives a real permission boundary later — a role serving results publicly can be
granted access to `placement`+`common` only, never `scraped`'s raw PII.

## `engine`/EDBO data-source status (see `REFACTOR_PLAN.md` for full detail)

**Confirmed empirically (not speculation):** EDBO's institution list moved from the old
`registry.edbo.gov.ua/api/universities/?exp=json` (now 404s) to
`registry.edbo.gov.ua/api/opendata/universities?rg=<region>&ut=<category>&exp=xlsx` — and the response is now a
real `.xlsx` file download, not JSON. No Cloudflare challenge was encountered fetching it. This means
institution-fetching needs an actual spreadsheet parser (`calamine` is the standard Rust crate) and a
redesigned DTO, not just a URL change — a bigger change than initially assumed. Exact semantics of `rg`/`ut`
(do they need to be looped over to get everything, or is there an "all" value) are still unconfirmed.

**Hard rule:** never use `vstup.osvita.ua` (or any non-EDBO source) as a data source for this project, even as a
fallback if EDBO scraping proves difficult. EDBO only — explicit project-owner decision, not a default to
reconsider.

## `server` crate

Boot sequence in `server/src/main.rs`: load `Config` from `config.toml` next to the running executable
(`server/src/config.rs` — auto-creates a default template and returns an error on first run if missing) →
validate into `RuntimeSettings` (`server/src/settings.rs`, requires non-empty `database_url`) → initialize the
`fern`-based `Logger` (`server/src/logs.rs`). The root `config.toml` is a dev sample (gitignored). `server`
does not yet depend on `scraper`/`model`/`placement` — see `REFACTOR_PLAN.md` for the wiring plan.

## Docker / local Postgres

- `docker-compose.dev.yaml`: Postgres only, for local development (`docker compose -f docker-compose.dev.yaml up -d`).
- `docker-compose.prod.yaml`: Postgres + the `server` app image, mounting `./config.toml` into the container.
- `Dockerfile`: multi-stage build, only ever builds `--package server --release`.
