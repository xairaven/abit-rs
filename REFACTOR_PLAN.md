# abit-rs: architecture and build plan

_Status as of 2026-08-16. Living planning document — update it as steps complete or the
approach changes; it's not a historical record. Code examples are illustrative shape, not
verbatim code to paste — the user is writing the implementation by hand._

## Context

`abit-rs` is being rebuilt into a CLI tool to calculate state-funded ("budgetary")
master's-degree scholarship placement for Ukraine's 2026 admissions cycle. Last year's
attempt (`edbo_core`) only got as far as an incomplete scraper/parser — the
placement/allocation algorithm was never finished.

**Current state:** `edbo_core/` at the repo root is a complete, standalone reference copy
of last year's fully-working implementation — **deliberately not a workspace member**,
exists purely to consult and port logic from. The workspace (`Cargo.toml` members) is now
`["server", "scraper", "model"]`:
- `scraper` (renamed from `engine`, itself renamed from `edbo_core`) — data acquisition +
  persistence. Currently a blank skeleton: `config.rs`, `schemas`, `errors.rs`,
  `scraper.rs`, `lib.rs` only. Internal type names (`EngineConfig`, `EngineError`) still
  reflect the pre-rename name — cosmetic, rename to `ScraperConfig`/`ScraperError`
  whenever convenient, not urgent.
- `model` — scaffolded but empty (`lib.rs` has no content, `Cargo.toml` has no
  dependencies yet). Will hold shared domain types once populated.
- `placement` (the allocation algorithm) — **not created yet**, planned.
- `server` — unchanged, doesn't depend on any of the above yet.

`docs/dbml/schema.dbml` (moved from the old root-level `dbml/core_schema.dbml`)
documents the scraped-data + common/reference tables; a schema doc for the eventual
`placement` tables doesn't exist yet.

### EDBO 2025→2026 obstacle — de-risking research

**Cloudflare: confirmed not the obstacle**, at least for the paths tested. A live check
against `vstup.edbo.gov.ua`/`registry.edbo.gov.ua` (`curl`, browser UA, no JS) found no
challenge on 3 read paths, and a real browser network-tab capture of an actual "get all
universities" click also sailed through with a clean 200 — no `cf-ray`, no challenge
markup, even though Turnstile is referenced in the site's CSP (configured *somewhere*,
just not triggering here).

**Confirmed empirically: the institution-data endpoint moved and changed shape, not just
URL.** Captured via real browser network tab clicking "get all universities" on
`registry.edbo.gov.ua/vishcha-osvita`:
```
GET https://registry.edbo.gov.ua/api/opendata/universities?rg=0&ut=1&exp=xlsx
→ 308 (trailing-slash redirect) → 200 OK
content-type: application/vnd.openxmlformats-officedocument.spreadsheetml.sheet
content-disposition: attachment; filename="Заклад вищої освіти <date>.xlsx"
```
Old code hit `registry.edbo.gov.ua/api/universities/?exp=json` (now 404s, confirmed via
direct `curl`) — the path gained an `opendata` segment, and **the response is now a real
`.xlsx` file, not JSON.** This is bigger than a URL fix: institution-fetching needs an
actual spreadsheet parser (`calamine` is the standard Rust crate for reading `.xlsx`) and
a DTO redesigned around spreadsheet columns, not JSON field names. Open questions to
resolve by more browser exploration before implementing:
- `ut` (category) and `rg` (region) — old code called the JSON version with both filters
  omitted (`None`) to get *all* institutions unfiltered. The new endpoint was captured
  with `ut=1&rg=0` — unconfirmed whether these are required, what values exist, or
  whether getting *all* institutions now requires looping over every `ut`/`rg` value and
  merging results. Check the UI's own filter controls on `vishcha-osvita` for the actual
  value range, or try omitting each param.
- Whether offers/applications moved to the same `/api/opendata/...` + xlsx-export pattern
  — worth checking for an equivalent export button before assuming the old `/offer/{id}`
  HTML-scraping / `/offer-requests/` POST-pagination approach still works as-is.

Corroborating research from `abit-assistant`'s (https://github.com/OlexiyOdarchuk/abit-assistant)
`brain/06 — EDBO Research.md` independently confirms the Next.js rewrite and that the old
AJAX endpoints are "non-functional for current data." Two things from that research still
apply:
- **Crypto cross-validation:** abit-assistant independently reverse-engineered the same
  AES-CBC salt formula `edbo_core/src/crypto.rs` already uses (`salt = "v" + (number ×
  (7500 − prsid))`, key = SHA256(salt)[:32 hex], IV = SHA256("2025")[:16 hex]) via
  Playwright capture of the archived `vstup2025/js/functions.js`. Combined with
  `crypto.rs`'s own 3 passing golden-value tests, port it unchanged.
- **abit-assistant has not built or live-tested EDBO scraping for 2026 itself** — paused
  pending their own campaign launch, and their battle-tested sources are
  `vstup.osvita.ua`/`abit-poisk.org.ua`, which **this project is explicitly forbidden from
  using** (hard rule: EDBO only, no fallback, ever). Only their methodology transfers: if
  more endpoints turn out broken, a Playwright/headless-browser capture of real traffic
  (their `tools/edbo-reverse/capture.py` + `analyze.py` pattern) is the recommended way to
  find the new shape — same technique that just found the opendata/xlsx endpoint by hand.

## Crate topology

```
model  (plain domain types — Institution, Offer, Application, Applicant, lookup enums,
        and eventually PlacementResult. Minimal deps: serde, thiserror, strum,
        strum_macros, num_enum, bigdecimal. NO reqwest/sqlx/aes/regex.)
  ^                              ^
  |                              |
scraper                     placement
(fetches EDBO,               (pure allocation algorithm,
 persists to Postgres,        reads model types, writes
 owns dto + crypto + api)     PlacementResult)
  ^                              ^
  |                              |
  +----------- server -----------+
  (orchestrates: run scraper, then run placement,
   eventually serves placement results to users)
```

**Why split `model` out at all:** `placement` needs to read the same domain types
`scraper` produces, but needs none of `scraper`'s other dependencies (HTTP, crypto, regex
HTML-scraping). Cargo compiles a whole crate's dependency graph as a unit — depending on
all of `scraper` just to see its structs would drag all of that in for no reason. This is
the same test that justified the `dto`/`model` split one level down: real, independent
consumers with genuinely different dependency needs, not abstraction for its own sake.

**Orphan-rule consequence:** today, `impl TryFrom<InstitutionDto> for Institution` lives
in `model/institution.rs`. Once `Institution` moves to the separate `model` crate while
`InstitutionDto` stays in `scraper`, that `impl` becomes illegal — Rust's orphan rule
requires either the trait or the implemented type to be local to the crate doing the
`impl`, and neither `TryFrom` (std) nor `Institution` (now in `model`) would be local to
`scraper`. Fix: replace the trait impl with a plain free function living in `scraper`
(which owns `dto` and depends on `model`, so it can see both), e.g.
`fn institution_from_dto(dto: InstitutionDto) -> Result<model::Institution, ModelError>`.
Finer-grained conversions that stay entirely within `model` (e.g.
`InstitutionCategory::try_from(i16)`) are unaffected — this only hits the
whole-DTO-struct-to-whole-domain-struct conversions.

## Database: one Postgres database, three schemas

Not three databases — Postgres cannot join or foreign-key across separate databases
without extensions (`postgres_fdw`/`dblink`), which is real operational overhead for
something `placement` would need constantly. Schemas (namespaces within one database) join
and FK freely, need only one connection/pool, and migrations are just SQL
(`CREATE SCHEMA foo;` is a normal migration statement) — so they give the same logical
separation with none of the cross-database cost.

Three schemas, matching who writes what:
- **`common`** — reference/lookup tables that both `scraper` and `placement` (and later
  `server`, for labeling results) read: `institution`, `region`, `degree`, `study_form`,
  `offer_type`, `ownership_form`, `institution_category`, `knowledge_field`, `speciality`,
  `application_status`. Written by `scraper`.
- **`scraped`** — EDBO input data: `offer`, `offers_institutions`, `application`,
  `applicant`. Written by `scraper`; read-only for `placement`.
- **`placement`** — algorithm output (design TBD when that phase starts, something like
  a `result` table: which applicant got which offer, or none). Written by `placement`;
  read by `server` to serve to users.

Nothing left in bare `public` — every table lives in one of the three named schemas, for
clarity.

**Why this is worth the (small) extra verbosity of schema-qualifying table names in raw
SQL:** schemas are a real Postgres permission boundary (`GRANT`/`REVOKE` per schema per
role). Given `server` is meant to eventually serve placement results to end users, this
means a role that only serves results publicly can eventually be granted access to
`placement`+`common` and nothing in `scraped` — so a bug in the results-serving path can't
leak raw scraped PII (decrypted applicant names, grades). Not a hypothetical concern for
this project; it's the stated end goal.

**Migration ownership:** `scraper`'s embedded migrations (`scraper/migrations/`) own
`common`+`scraped` (create both schemas + all their tables) — this is what
`scraper/src/database.rs`'s `Database::init` already runs via `sqlx::migrate!()`. The
future `placement` crate embeds its own migrations owning just the `placement` schema,
with its own `sqlx::migrate!()` call against the same database. Both run fine against one
DB as long as migration version numbers don't collide between the two crates' migration
folders (trivial to ensure — timestamp-based naming from `sqlx migrate add` already avoids
this by construction).

## Architecture within `scraper`: dropping the Service/Repository trait pattern

`edbo_core` uses a `Service` trait (`new(&Database)`) wrapping a `Repository` trait
(`new(&Database)`, `is_empty()`), one impl of each per entity — 5 `services/*.rs` + 13
`repository/*.rs` files, plus `EnumService`, a struct holding 9 repository fields with a
`build()` method that's 9 copy-pasted `if is_empty { create } else { skip }` blocks.

**Diagnosis: neither trait is ever used polymorphically.** Every call site names the
concrete type directly — nothing takes a `Box<dyn Repository>` or is generic over
`Service`. The usual reasons to separate data-access from orchestration behind traits are:
(a) mock the repository to unit-test business logic without a DB, (b) swap storage
backends, (c) separate reasons-to-change. Checked against this codebase: (a) isn't
happening (no mocks, no alternate impls, ever), (b) isn't realistic (`sqlx::query!` is
already Postgres-schema-bound at compile time), so only (c) has real merit — and (c)
doesn't require traits or separate files, just separate *functions*.

### Lookup-table seeding: macro-generated functions, not repository objects

`edbo_core`'s 9 "enum-seed" repositories (`degree`, `region`, `study_form`, `offer_type`,
`ownership_form`, `institution_category`, `status`, plus `knowledge_field`/`speciality`
which don't fit the simple shape) are structurally identical: check `is_empty()`, if so
insert every variant of a Rust enum via `strum::IntoEnumIterator`. Replace with one
`macro_rules!` template generating a plain function per table — keeps full
`sqlx::query!` compile-time schema checking (macro_rules! expands to a literal token
stream before `sqlx::query!` ever runs, so each expansion is independently checked), with
no trait, no struct, no `::new()`:

```rust
macro_rules! impl_enum_seed {
    ($fn_name:ident, $enum_ty:ty, $table:literal, $insert_sql:literal) => {
        pub async fn $fn_name(pool: &PgPool) -> Result<(), EngineError> {
            let is_empty: bool = sqlx::query_scalar!(
                concat!("SELECT NOT EXISTS (SELECT 1 FROM ", $table, ")")
            )
            .fetch_one(pool).await?.unwrap_or(true);

            if !is_empty { return Ok(()); }

            for variant in <$enum_ty>::iter() {
                sqlx::query!($insert_sql, variant as i16, variant.to_string())
                    .execute(pool).await?;
            }
            Ok(())
        }
    };
}

impl_enum_seed!(seed_degree, Degree, "common.degree",
    "INSERT INTO common.degree (id, description) VALUES ($1, $2)");
impl_enum_seed!(seed_region, Region, "common.region",
    "INSERT INTO common.region (id, description) VALUES ($1, $2)");
// ...5 more, one line each

pub async fn seed_lookup_tables(pool: &PgPool) -> Result<(), EngineError> {
    seed_degree(pool).await?;
    seed_region(pool).await?;
    // ...
    seed_knowledge_field(pool).await?;   // own hand-written fn, doesn't fit the macro shape
    seed_speciality(pool).await?;        // ditto — keep edbo_core's `define_specialities!`-driven catalog
    Ok(())
}
```
(Table names above are schema-qualified per the `common` schema decision above.)

### Real entities: one module per entity, layered by visibility not by trait

For the 5 "real" entities (`institution`, `offer`, `offers_university`, `application`,
`applicant`), merge each `service`+`repository` pair into a single module. Separation of
concerns is preserved by `pub` vs private function visibility, not by a trait/type
boundary:

```rust
// scraper/src/entities/offer.rs
pub async fn get(
    pool: &PgPool, offers_with_institutions: &mut [OffersUniversity],
) -> Result<Vec<Offer>, EngineError> {
    if is_empty(pool).await? {
        let list = api::offers::list(offers_with_institutions).await?;
        for offer in &list {
            insert(pool, offer).await?;
        }
        Ok(list)
    } else {
        find_all(pool).await
    }
}

async fn is_empty(pool: &PgPool) -> Result<bool, EngineError> { /* SQL, schema.table = scraped.offer */ }
async fn insert(pool: &PgPool, offer: &Offer) -> Result<(), EngineError> { /* SQL */ }
async fn find_all(pool: &PgPool) -> Result<Vec<Offer>, EngineError> { /* SQL, shared row-mapping
    helper if find_by_id also exists in this file — see edbo_core's repository/institution.rs
    and repository/offer.rs for the row-mapping duplication this avoids */ }
```

5 files instead of 10, no traits, no `Service::new(&db)`/`Repository::new(&db)`
construction step. If a module ever gets too large, split it into a `mod sql;` submodule
at that point — don't pre-split for a size problem that doesn't exist yet.

### Error architecture, flattened

No more `RepositoryError` wrapper type (no "repository" layer left to own it). One flat
`EngineError` in `errors.rs` (rename to `ScraperError` alongside the `EngineConfig` →
`ScraperConfig` cleanup mentioned above, whenever convenient):

```rust
#[derive(Debug, Error)]
pub enum EngineError {
    #[error("API Error. {0}")]
    Api(#[from] ApiError),
    #[error("Database Error. {0}")]
    Db(#[from] DbError),           // already exists today
    #[error("Model Error. {0}")]
    Model(#[from] ModelError),
    #[error("SQL Error. {0}")]
    Sql(#[from] sqlx::Error),
    #[error("JSON Error. {0}")]
    Json(#[from] serde_json::Error),  // needed for applicant.grade_components JSONB column
}
```

### `Scraper::process()` becomes a flat list of calls, not object construction

```rust
pub async fn process(&self) -> Result<Context, EngineError> {
    let db = Database::init(&self.config).await?;

    lookup::seed_lookup_tables(&db.pool).await?;
    let institutions = entities::institution::get(&db.pool).await?;
    let mut offers_with_institutions = entities::offer_university::get(&db.pool).await?;
    let offers = entities::offer::get(&db.pool, &mut offers_with_institutions).await?;
    let (applications, applicants) = entities::application::get(&db.pool, &offers).await?;

    Ok(Context { applicants, applications, institutions, offers, offers_with_institutions })
}
```

## Schema-drift fixes to apply while porting from `edbo_core`

Found by comparing `docs/dbml/schema.dbml`, `scraper/migrations/001_setup.sql` (once
restored — currently the migration file lives untouched at
`edbo_core/migrations/001_setup.sql`, needs porting into `scraper/migrations/` with these
fixes applied and the schema-qualification above), and `edbo_core/src/model/*.rs` — port
these fixed, don't copy the old types verbatim:

| Drift | Fix |
|---|---|
| `institution.id`/`parent_id`: `i16` in old model vs SQL `INTEGER` | New model → `i32`/`Option<i32>` |
| Enum repr `i8` (7 files + `priority.rs`) vs SQL `SMALLINT`(i16) | New model → `#[repr(i16)]`; matches the old repository code's own `// TODO: Fix Warning — casting i16 to i8 may truncate` comments |
| `application.grade`: DBML `float` / SQL `DECIMAL(10,3)` / old model `f32` | New model → `bigdecimal::BigDecimal` (already a planned dependency); convert once from `ApplyRequestDto.kv: f32` at the DTO→model boundary, not in SQL-access code |
| Migration missing FK `offers_institutions.offer_id → offer.id` | Add the constraint directly into the new `scraper/migrations/001_setup.sql` — DB is disposable/re-seedable (fetch-or-cache design means nothing is hand-curated), so a clean migration beats a patch |
| DBML doc drift: `grade` type, `priority_id` vs `priority_code`, `institution_id` vs `university_id`, missing composite-PK annotations, stale `user_id` nullability | Update `docs/dbml/schema.dbml` to match the migration (SQL is source of truth) — follow-up, non-blocking |

## Build order

1. **`model` crate**: populate with domain types ported from `edbo_core/src/model/*.rs`,
   schema-drift fixes applied, `ModelError`. Dependencies: serde, thiserror, strum,
   strum_macros, num_enum, bigdecimal — confirm nothing else sneaks in.
2. **`scraper` crate**, leaf-to-root, now depending on `model`:
   - `errors.rs` — the flattened `EngineError`/`ScraperError` above.
   - `crypto.rs` — port as-is (verified correct).
   - `dto/*.rs` — port as-is; for institution data, redesign around the confirmed
     `.xlsx` opendata response shape (needs `calamine`), not the old JSON shape.
   - Free conversion functions (`fn institution_from_dto(...) -> Result<model::Institution, ModelError>`
     etc.) replacing the old `TryFrom<Dto> for Model` impls, per the orphan-rule note above.
   - `request.rs` + `api.rs`/`api/*.rs` — port with endpoint fixes as EDBO exploration
     confirms them (institution endpoint confirmed changed; offers/applications
     unconfirmed, check before assuming the old approach works).
   - `lookup.rs` (new) — the macro-generated seed functions, schema-qualified to `common`.
   - `entities/*.rs` (new, replacing `services/`+`repository/`) — merged modules, one per
     entity, schema-qualified to `scraped` (or `common` for `institution`).
   - `context.rs` — port as-is.
   - `scraper.rs` — the flat pipeline shown above, `process()` returning
     `Result<Context, EngineError>`.
   - `lib.rs` — declare all new modules.
   - `migrations/001_setup.sql` — new migration creating `common`+`scraped` schemas and
     all their tables, schema-drift-fixed.
3. Build after each step (`cargo build --workspace`) rather than all at once.
4. `placement` crate — separate future phase, not started. Depends on `model` only.

## Wire `server` to call `scraper`

Add to `server/Cargo.toml`:
```toml
[dependencies]
scraper = { path = "../scraper" }
```

In `server/src/main.rs`, after the existing `Logger::from_settings(...)` block:
```rust
use scraper::{EngineConfig, Scraper};   // rename to ScraperConfig/Scraper once the
                                         // internal renames above happen
// ...
let engine_config = EngineConfig {
    database_url: runtime_settings.database_url.clone(),
};

let _context = Scraper::new(&engine_config)
    .process()
    .await
    .unwrap_or_else(|error| {
        log::error!("Scraper process failed. {error}");
        std::process::exit(1);
    });

log::info!("Process finished successfully.");
```
Matches the file's existing `unwrap_or_else` + exit(1) pattern; uses `log::error!` instead
of `eprintln!` since this call happens after the logger is initialized. `_context` is
discarded for now — nothing downstream consumes scraped data yet. Once `placement` exists,
`server` will also depend on it and call it after `scraper` finishes.

## Explicitly out of scope for this pass

- Building the `placement` crate itself (algorithm design) — separate future phase.
- Further EDBO endpoint discovery beyond institutions (offers/applications) — check before
  assuming the old approach works, but don't block this pass on it.
- **Never** `vstup.osvita.ua` or any non-EDBO source — off the table by explicit project
  rule regardless of how EDBO scraping goes.
- `docs/dbml/schema.dbml` doc fixes and a `placement`-schema dbml doc — small
  follow-ups, not blocking.
- `api/*.rs` request-boilerplate deduplication (client/headers/ticker/retry-loop) — small,
  but interleaved with delicate per-endpoint parsing; better done once/if endpoint changes
  force a rewrite of that code anyway.

## Verification

1. After each build-order step: `cargo build --workspace` — confirm no errors before
   moving to the next layer.
2. Once `crypto` is ported and declared: `cargo test --package scraper crypto::tests` —
   should pass 3 golden-value tests.
3. `cargo clippy --workspace --all-targets` — confirm no unexpected pedantic/nursery
   warnings in the newly-written code.
4. End-to-end: bring up `docker-compose.dev.yaml`'s `db`, run `cargo run --package server`
   against it, and watch how far the real pipeline gets against live EDBO — specifically
   whether the redesigned institution fetch (xlsx-based) succeeds, and whether
   offer/application scraping succeeds or needs the same kind of endpoint rework. This
   result directly answers how much further EDBO-side exploration is needed before the
   scraper is genuinely done.
