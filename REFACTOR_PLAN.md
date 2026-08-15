# `engine`: architecture and build plan

_Status as of 2026-08-13. Living planning document — update it as steps complete or the
approach changes; it's not a historical record. Code examples below are illustrative
shape, not verbatim code to paste — the user is writing the implementation by hand._

## Context

`abit-rs` is being rebuilt into a CLI tool to calculate state-funded ("budgetary")
master's-degree scholarship placement for Ukraine's 2026 admissions cycle. Last year's
attempt (`edbo_core`) only got as far as an incomplete scraper/parser — the
placement/allocation algorithm was never finished. That's a known future phase, out of
scope here.

**Current state:** `edbo_core/` has been restored at the repo root as a complete,
standalone reference copy of last year's fully-working implementation — it is
**deliberately not a workspace member** and not meant to compile or be built on directly;
it exists purely to consult and port logic from. `engine/` is a genuinely blank slate:
`engine/src/` contains only `config.rs` (`EngineConfig { database_url }`), `database.rs`
(`Database::init`, admin-DB-free via `sqlx::migrate::MigrateDatabase`), `errors.rs`
(currently a single-variant `EngineError`), `scraper.rs` (a `Scraper` struct whose
`process()` does nothing but `Database::init` so far), and `lib.rs`. Everything else is
being **designed fresh**, using `edbo_core` as reference material, not mechanically
restored.

**EDBO 2025→2026 obstacle — de-risking research (still valid):** a live check against
`vstup.edbo.gov.ua` and `registry.edbo.gov.ua` (plain `curl`, browser User-Agent, no JS)
found **no Cloudflare challenge** on any of 3 tested paths (an offer page, the homepage, a
registry API path) — no `cf-ray`, no challenge markup. Turnstile is referenced in the
site's CSP (configured *somewhere*), but didn't trigger on these paths. What *is*
confirmed: the site has been rebuilt on **Next.js** since last year (`X-Powered-By:
Next.js` on every response; the old registry JSON endpoint `/api/universities/?exp=json`
now 404s). So the real 2026 blocker looks more like **moved/restructured API endpoints**
from a full site rewrite, not bot-protection — though the untested case that matters most
is the high-volume paginated `POST /offer-requests/` loop (applications scraping), the
pattern most likely to actually trigger rate-limiting. **Don't assume a browser-driven
Cloudflare bypass is needed until the rebuilt pipeline has actually been run against live
EDBO and the failure mode observed directly.**

Corroborating this independently: `abit-assistant`'s (https://github.com/OlexiyOdarchuk/abit-assistant)
`brain/06 — EDBO Research.md` states the portal "shifted from a jQuery/Handlebars
architecture to a Next.js SPA" and that the old AJAX endpoints are "non-functional for
current data" — matching our own 404 exactly. Two more things from that research:
- **Crypto cross-validation:** abit-assistant independently reverse-engineered the same
  AES-CBC salt formula `edbo_core/src/crypto.rs` already uses — `salt = "v" + (number ×
  (7500 − prsid))`, key = SHA256(salt)[:32 hex], IV = SHA256("2025")[:16 hex] — via
  Playwright capture of the archived `vstup2025/js/functions.js`. Combined with
  `crypto.rs`'s own 3 passing golden-value tests against real captured ciphertext, this is
  strong confirmation the crypto logic can be ported to `engine` unchanged.
- **abit-assistant has not actually built or live-tested EDBO scraping for 2026** — their
  own roadmap marks it "paused pending the 2026 campaign launch." Their battle-tested data
  sources are `vstup.osvita.ua` + `abit-poisk.org.ua` — **sources this project is
  explicitly forbidden from using** (hard rule: EDBO only, no osvita.ua fallback, ever,
  regardless of how EDBO scraping goes). Only their methodology transfers: if/when the
  rebuilt pipeline is confirmed broken against live EDBO, the recommended next step is a
  Playwright/headless-browser-driven capture of real XHR traffic (their
  `tools/edbo-reverse/capture.py` + `analyze.py` pattern) to learn the new endpoint shape —
  a future-phase concern, not part of this build.

## Architecture: dropping the Service/Repository trait pattern

`edbo_core` uses a `Service` trait (`new(&Database)`) wrapping a `Repository` trait
(`new(&Database)`, `is_empty()`), one impl of each per entity — 5 `services/*.rs` + 13
`repository/*.rs` files, plus `EnumService`, a struct holding 9 repository fields with a
`build()` method that's 9 copy-pasted `if is_empty { create } else { skip }` blocks.

**Diagnosis: neither trait is ever used polymorphically.** Every call site
(`OfferRepository::new(&db)`, `EnumService::new(&db)`, ...) names the concrete type
directly — nothing takes a `Box<dyn Repository>` or is generic over `Service`. The usual
reasons to separate data-access from orchestration behind traits are: (a) mock the
repository to unit-test business logic without a DB, (b) swap storage backends, (c)
separate reasons-to-change. Checked against this codebase: (a) isn't happening (no mocks,
no alternate impls anywhere, ever), (b) isn't realistic (`sqlx::query!` is already
Postgres-schema-bound at compile time), so only (c) has real merit — and (c) doesn't
require traits or separate files, just separate *functions*.

### Decision 1 — lookup-table seeding: macro-generated functions, not repository objects

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

impl_enum_seed!(seed_degree, Degree, "degree",
    "INSERT INTO degree (id, description) VALUES ($1, $2)");
impl_enum_seed!(seed_region, Region, "region",
    "INSERT INTO region (id, description) VALUES ($1, $2)");
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

### Decision 2 — entity fetch-or-cache: one module per entity, layered by visibility not by trait

For the 5 "real" entities (`institution`, `offer`, `offers_university`, `application`,
`applicant`), merge each `service`+`repository` pair into a single module. Separation of
concerns is preserved by `pub` vs private function visibility, not by a trait/type
boundary:

```rust
// engine/src/entities/offer.rs
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

async fn is_empty(pool: &PgPool) -> Result<bool, EngineError> { /* SQL */ }
async fn insert(pool: &PgPool, offer: &Offer) -> Result<(), EngineError> { /* SQL */ }
async fn find_all(pool: &PgPool) -> Result<Vec<Offer>, EngineError> { /* SQL, shared row-mapping
    helper if find_by_id also exists in this file — see edbo_core's repository/institution.rs
    and repository/offer.rs for the row-mapping duplication this avoids */ }
```

5 files instead of 10, no traits, no `Service::new(&db)`/`Repository::new(&db)`
construction step. If a module ever gets too large, split it into a `mod sql;` submodule
at that point (e.g. `entities/offer/mod.rs` + `entities/offer/sql.rs`) — don't pre-split
for a size problem that doesn't exist yet.

### Decision 3 — error architecture, flattened

No more `RepositoryError` wrapper type (there's no "repository" layer anymore to own it).
One flat `EngineError` in `errors.rs`:

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

Found by comparing `dbml/core_schema.dbml`, `engine/migrations/001_setup.sql`, and
`edbo_core/src/model/*.rs` — port these fixed, don't copy the old types verbatim:

| Drift | Fix |
|---|---|
| `institution.id`/`parent_id`: `i16` in old model vs SQL `INTEGER` | New model → `i32`/`Option<i32>` |
| Enum repr `i8` (7 files + `priority.rs`) vs SQL `SMALLINT`(i16) | New model → `#[repr(i16)]`; matches the old repository code's own `// TODO: Fix Warning — casting i16 to i8 may truncate` comments |
| `application.grade`: DBML `float` / SQL `DECIMAL(10,3)` / old model `f32` | New model → `bigdecimal::BigDecimal` (already a dependency); convert once from `ApplyRequestDto.kv: f32` at the DTO→model boundary, not in SQL-access code |
| Migration missing FK `offers_institutions.offer_id → offer.id` | Add the constraint directly into `engine/migrations/001_setup.sql` — DB is disposable/re-seedable (fetch-or-cache design means nothing is hand-curated), so editing in place beats a patch migration |
| DBML doc drift: `grade` type, `priority_id` vs `priority_code`, `institution_id` vs `university_id`, missing composite-PK annotations, stale `user_id` nullability | Update `dbml/core_schema.dbml` to match the migration (SQL is source of truth) — follow-up, non-blocking |

## Build order

Port from `edbo_core` into `engine`, leaf-to-root, rewriting to the new shape as you go
(not copy-paste):

1. `errors.rs` — the flattened `EngineError` above.
2. `crypto.rs` — port as-is (verified correct, see research above).
3. `dto/*.rs` — port as-is (pure serde structs, no old-type coupling).
4. `model/*.rs` — port with the schema-drift fixes above applied.
5. `request.rs` + `api.rs`/`api/*.rs` — port as-is; these don't need the
   Service/Repository rework (that pattern was never applied here — `api/*.rs` already
   deferred cleanup in the earlier duplication review, since its client/header/ticker
   boilerplate is small and interleaved with delicate per-endpoint parsing logic; still
   defer touching that shape until the Cloudflare/endpoint-discovery phase, if it happens).
6. `lookup.rs` (new) — Decision 1's macro-generated seed functions.
7. `entities/*.rs` (new module, replacing `services/`+`repository/`) — Decision 2's
   merged modules, one per entity.
8. `context.rs` — port as-is (plain data bag, no old-type coupling).
9. `scraper.rs` — the flat pipeline shown above; widen `process()`'s return type to
   `Result<Context, EngineError>`.
10. `lib.rs` — declare all new modules.

Build after each step (`cargo build --workspace`) rather than all at once.

## Wire `server` to call `engine`

Add to `server/Cargo.toml`:
```toml
[dependencies]
engine = { path = "../engine" }
```

In `server/src/main.rs`, after the existing `Logger::from_settings(...)` block:
```rust
use engine::{EngineConfig, Scraper};
// ...
let engine_config = EngineConfig {
    database_url: runtime_settings.database_url.clone(),
};

let _context = Scraper::new(&engine_config)
    .process()
    .await
    .unwrap_or_else(|error| {
        log::error!("Engine process failed. {error}");
        std::process::exit(1);
    });

log::info!("Process finished successfully.");
```
Matches the file's existing `unwrap_or_else` + exit(1) pattern; uses `log::error!` instead
of `eprintln!` since this call happens after the logger is initialized. `_context` is
discarded for now — nothing downstream consumes scraped data yet (future phase, same as
placement/allocation).

## Explicitly out of scope for this pass

- EDBO Cloudflare/anti-bot handling beyond the empirical check already done — validate
  further only once the pipeline runs end-to-end again. If it turns out to be a real
  blocker, the recommended next step is a Playwright/headless-browser-driven capture of
  live XHR traffic (see abit-assistant research above) — **never** a switch to
  `vstup.osvita.ua` or any non-EDBO source; that's off the table by explicit project rule.
- Placement/allocation algorithm.
- DBML doc fixes (small follow-up, not blocking).
- `api/*.rs` request-boilerplate deduplication (client/headers/ticker/retry-loop) — small,
  but interleaved with delicate per-endpoint parsing; better done once/if the Cloudflare
  phase forces a transport change anyway.

## Verification

1. After each build-order step: `cargo build --workspace` — confirm no errors before
   moving to the next layer.
2. Once `crypto` is ported and declared: `cargo test --package engine crypto::tests` —
   should pass 3 golden-value tests.
3. `cargo clippy --workspace --all-targets` — confirm no unexpected pedantic/nursery
   warnings in the newly-written code.
4. End-to-end: bring up `docker-compose.dev.yaml`'s `db`, run `cargo run --package server`
   against it, and watch how far the real pipeline gets against live EDBO — specifically
   whether `institution`/`offer`/`offer_university` (GET-based) succeed, and whether the
   `application` POST-pagination loop (the untested, highest-risk endpoint) succeeds or
   gets rate-limited/blocked. This result directly answers whether a 2026 Cloudflare-bypass
   phase is even necessary, or whether it's just a matter of updated endpoint paths.