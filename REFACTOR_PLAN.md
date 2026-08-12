# `engine` re-wiring: from orphaned skeleton to clean, working 2025-equivalent scraper

_Status as of 2026-08-13. This is a living planning document for the `edbo_core` → `engine`
rewrite — update it as steps complete or the approach changes; it's not a historical record._

## Context

`abit-rs` is being rebuilt into a CLI tool to calculate state-funded ("budgetary")
master's-degree scholarship placement for Ukraine's 2026 admissions cycle. Last year's
attempt only got as far as an incomplete scraper/parser (`edbo_core`), and the user is now
doing a ground-up rewrite, renamed to `engine`.

Mid-rewrite, `engine/src/lib.rs` currently declares only a new minimal skeleton (`config`,
`database`, `errors`, `scraper`), while the entire previous, fully-working implementation
(`api/`, `model/`, `repository/`, `services/`, `crypto.rs`, `context.rs`, `dto/`,
`request.rs`) sits on disk unreferenced by any `mod` declaration — it doesn't affect
compilation, but it's also not doing anything. This document plans getting it back to a
clean, working state functionally equivalent to what worked a year ago, before tackling
whatever's changed on EDBO's side for 2026.

**Why this matters now, concretely:** every orphaned file references a `CoreError`
aggregate error type that no longer exists (the new `errors.rs` has only one variant,
`Db`). That's the single blocking reason none of this code can be turned back on yet.
Separately, three real DB-schema/model type mismatches exist that would surface as
compile errors or silent truncation bugs the moment repository code (which already has
`// TODO: Fix Warning` comments flagging exactly this) gets re-wired.

**De-risking finding worth acting on early:** a live check against
`vstup.edbo.gov.ua` and `registry.edbo.gov.ua` (plain `curl`, browser User-Agent, no JS)
found **no Cloudflare challenge** on any of 3 tested paths (an offer page, the homepage, a
registry API path) — no `cf-ray`, no challenge markup. Turnstile is referenced in the
site's CSP (so it's configured *somewhere*), but didn't trigger here. More notably, the
old registry JSON endpoint (`/api/universities/?exp=json`) now returns a Next.js 404 —
`X-Powered-By: Next.js` appeared on every response. This means the real 2026 blocker
may be **moved/restructured endpoints from a full site rewrite**, not bot-protection, at
least for the read-only GET paths. The one untested case that matters most is the
high-volume paginated `POST` loop against `/offer-requests/` (used to scrape
applications) — that's the pattern most likely to actually trigger rate-limiting/
challenge behavior, being by far the highest request-volume part of the old pipeline.
**Recommendation: re-wire and try the pipeline against live EDBO before assuming a
browser-driven Cloudflare bypass is needed at all** — it's possible only the URL paths
need updating.

**Corroborating research — abit-assistant's `brain/` dev notes:** reading
`brain/06 — EDBO Research.md`, `08 — Roadmap.md`, and `09 — Журнал.md` from
https://github.com/OlexiyOdarchuk/abit-assistant confirms the Next.js-rewrite finding
independently: their notes state the admissions portal "shifted from a jQuery/Handlebars
architecture to a Next.js SPA" and that the old AJAX endpoints
(`/university-search/`, `/offers-universities/`, `/offers-list/`) are "non-functional for
current data" against the new backend — matching our own recon's 404 on the registry
endpoint exactly. Two more useful things fell out of that research:
- **Crypto cross-validation:** abit-assistant independently reverse-engineered the same
  AES-CBC salt formula our `crypto.rs` already uses — `salt = "v" + (number × (7500 −
  prsid))`, key = SHA256(salt)[:32 hex], IV = SHA256("2025")[:16 hex] — via Playwright
  capture of the archived `vstup2025/js/functions.js`. Combined with `crypto.rs`'s own 3
  passing golden-value tests against real captured ciphertext, this is strong independent
  confirmation `crypto.rs` needs zero changes (reinforces step 2 of the re-wiring order
  below).
- **abit-assistant has not actually built or live-tested EDBO scraping for 2026** — per
  their own roadmap, EDBO integration is "paused pending the 2026 campaign launch,"
  blocked on capturing live XHR traffic during an active cycle. Their battle-tested data
  sources are `vstup.osvita.ua` + `abit-poisk.org.ua` — sources **this project is
  explicitly forbidden from using** (hard rule from the project owner: EDBO only, no
  osvita.ua fallback, regardless of how EDBO scraping goes). So there's no ready-made 2026
  EDBO endpoint map to borrow — only their methodology transfers: **if/when the re-wired
  pipeline is confirmed broken against live EDBO**, the recommended next step is the same
  one abit-assistant planned for itself — a Playwright/headless-browser-driven capture of
  real XHR traffic (their `tools/edbo-reverse/capture.py` + `analyze.py` pattern) to learn
  the new endpoint shape, rather than guessing from static HTML analysis. This is
  explicitly a future-phase concern, not part of this re-wiring pass.

## Recommended approach

Re-wire the orphaned code back into `engine/src/lib.rs` layer by layer (leaves before
roots), fixing the `CoreError` gap and the 3 schema-drift issues as part of the same pass,
and deduplicating only the 2-3 clusters where it's unambiguously cheap and safe to do so.
Defer Cloudflare-handling and the placement/allocation algorithm entirely — out of scope
for this pass.

### 1. Error architecture

Revive the old `CoreError` shape verbatim, renamed `EngineError`, as a flat aggregate:

```rust
// engine/src/errors.rs
#[derive(Debug, Error)]
pub enum EngineError {
    #[error("API Error. {0}")]
    Api(#[from] ApiError),
    #[error("Database Error. {0}")]
    Db(#[from] DbError),          // already exists today
    #[error("Model Error. {0}")]
    Model(#[from] ModelError),
    #[error("Repository Error. {0}")]
    Repository(#[from] RepositoryError),
}
```

`engine/src/repository.rs`'s `RepositoryResult<T>` aliases directly to
`Result<T, EngineError>` (as it did before, just renamed). No narrower per-layer error
type at the repository boundary — the old code already mixes `ModelError` (enum-parse
failures reading a row) and `RepositoryError::Sql` freely inside repository methods, and
`api/*.rs`'s top-level `list()` functions need the full aggregate since they mix HTTP
failures with model-conversion failures. Inventing a narrower boundary type here would add
a conversion layer that serves no real consumer in a solo project — restore what already
worked.

### 2. Re-wiring order (leaf → root; schema fixes land inside the `model` step)

1. **`errors.rs`** — add the 3 missing variants above. Nothing depends on it yet.
2. **`crypto.rs`, `dto/*.rs`** — declare via `mod` in `lib.rs`. Zero code changes: `crypto.rs`
   already verified standalone-correct (3 golden-value tests pass against real EDBO
   ciphertexts when isolated in a scratch crate), `dto/*.rs` has no old-type coupling.
3. **`model/*.rs` + `model.rs`** — fix schema drift here, before declaring `mod model;`:
   - `model/institution.rs`: `id: i16`/`parent_id: Option<i16>` → `i32`/`Option<i32>`
     (migration column is `INTEGER`; update the `TryFrom<InstitutionDto>` parse calls to
     match). `registration_year: Option<i16>` is correct as-is (matches `INT2`) — leave it.
   - Enum repr fix across 7 files (`model/{degree,status,study_form,institution_category,
     ownership_form,offer_type,region}.rs`) plus `model/priority.rs`: change
     `#[repr(i8)]` → `#[repr(i16)]`, and `priority.rs`'s hand-rolled `From<i8>`/`Into<i8>`
     impls (plus `Budgetary(i8)` → `Budgetary(i16)`) to `i16`. All 7 lookup columns are
     `SMALLINT` (i16) in the migration; the Rust side should mirror the DB, not the other
     way around — this is exactly what the repository layer's existing
     `// TODO: Fix Warning — casting i16 to i8 may truncate` comments are flagging, and
     `clippy::pedantic` (already enabled workspace-wide) will flag the lossy casts too.
   - `model/application.rs`: `grade: f32` → `grade: bigdecimal::BigDecimal` (matches the
     migration's `DECIMAL(10,3)` exactly; `bigdecimal` is already a dependency specifically
     for this). Do the `f32`→`BigDecimal` conversion **in `Application::try_from_dto`**
     (converting `ApplyRequestDto.kv: f32` once, at the DTO→model boundary — confirmed by
     reading `dto/application.rs`), not in the repository layer — that matches how every
     other model type in this codebase does its parsing/conversion at construction time,
     and lets `repository/application.rs` become a straight pass-through with no
     round-trip conversion at all.
   - Declare `mod model;` (all submodules) in `lib.rs`.
   - This is one logical commit ("fix model/DB type drift") — the institution-width fix
     and the enum-repr fix are both instances of "make Rust match the migration," so
     splitting them further buys nothing.
4. **`request.rs`, `api.rs` + `api/*.rs`** — declare via `mod` in `lib.rs`. No code changes
   beyond what step 3 already propagates through `TryFrom<Dto>` calls; `ApiError`,
   `ApiFetcherUrl`/`ApiFetcherForm`, and the rate-limit retry logic are self-contained.
5. **`repository.rs` + `repository/*.rs`** (13 files) — declare via `mod`. Changes:
   `CoreError` → `EngineError` rename throughout; delete the now-unnecessary lossy casts
   this schema fix makes obsolete (`row.id as i16` → `row.id`,
   `InstitutionCategory::try_from(row.category_id as i8)` →
   `InstitutionCategory::try_from(row.category_id)`, etc. — this is also where every
   `// TODO: Fix Warning` comment gets deleted along with the cast it was flagging); strip
   the grade round-trip conversion out of `repository/application.rs` entirely (grade is
   already `BigDecimal` after step 3, so it passes straight through). Split into 3
   sub-commits by fix-type (institution-width propagation / enum-repr propagation /
   grade propagation) since they're independent even though they touch overlapping files.
6. **`context.rs`** — declare via `mod`. Also decide: widen `Scraper::process()`'s return
   type from `Result<(), EngineError>` to `Result<Context, EngineError>` and return the
   built `Context` at the end, rather than silently discarding the scraped data (the whole
   point of running the scraper). Only caller today is the not-yet-written `server`
   wiring, so this is free to decide.
7. **`services.rs` + `services/*.rs`** (5 files) — declare via `mod`. Only change:
   `CoreError` → `EngineError` rename. Everything else (fetch-or-cache orchestration)
   operates on `model::*` types and is untouched by the schema fixes.
8. **`scraper.rs`** — uncomment the already-present pipeline sketch
   (`enum_service` → `institutions` → `offer_university` → `offer` → `applications` →
   build `Context`), adjust for the `Context`-returning signature from step 6.
9. **`lib.rs`** — finalize module list (add `api`, `context`, `crypto`, `dto`, `model`,
   `repository`, `services`, `request`), remove the `// TODO: W.I.P` comment, re-export
   `Context` alongside the existing `EngineConfig`/`Scraper` exports.
10. Build after each step (`cargo build --workspace`), not just at the end — 40+ files are
    touched in total; catching errors layer-by-layer is far more tractable than one
    big-bang restoration.

### 3. Schema-drift fixes (full list, including doc-only follow-ups)

| Drift | Fix | Blocking? |
|---|---|---|
| `institution.id`/`parent_id`: i16 vs SQL `INTEGER` | Rust → `i32`/`Option<i32>` | Yes — step 3 |
| Enum repr i8 vs SQL `SMALLINT`(i16), 7 files + priority.rs | Rust → `#[repr(i16)]` | Yes — step 3 |
| `application.grade`: DBML `float` / SQL `DECIMAL(10,3)` / Rust `f32` | Rust → `BigDecimal`; convert once in `try_from_dto` | Yes — step 3 |
| Migration missing FK `offers_institutions.offer_id → offer.id` | Add the constraint directly into `engine/migrations/001_setup.sql` (DB is disposable/re-seedable — see below — so editing in place beats a patch migration) | Real gap, low urgency — fix alongside step 3 or as its own tiny commit |
| DBML doc drift: `grade` type, `priority_id` vs `priority_code`, `institution_id` vs `university_id`, missing composite-PK annotations, stale `user_id` nullability | Update `dbml/core_schema.dbml` to match the migration (SQL is source of truth) | No — follow-up, non-blocking |

**Why editing `001_setup.sql` in place is safe:** the entire service/repository design is
fetch-or-cache (`is_empty()` → fetch from API or read from DB) — nothing in any table is
hand-curated, everything is re-derivable by truncating and re-running the pipeline. No
`.sqlx`/`sqlx-data.json` offline cache exists to invalidate either. This reasoning would
flip if a production instance with real scraped data existed already — nothing in the
repo suggests that's the case yet.

### 4. Deduplication scope — do 3 clusters now, defer 2

- **DO NOW — enum-seed repositories** (`repository/{degree,institution_category,
  knowledge_field,offer_type,ownership_form,region,speciality,status,study_form}.rs`, 9
  files): structurally identical `is_empty`/`create` shape, differing only in table name
  and enum type. Flatten via a `macro_rules!` (e.g. `impl_enum_repository!(...)`) in
  `repository.rs`, matching the style already established by `model/speciality.rs`'s
  `define_specialities!` macro — not a new abstraction pattern, applying an existing one.
  Doing this now reduces total edit surface (fix the `CoreError` rename and any lingering
  cast once, in the macro body, instead of 9 times).
- **DO NOW — row-mapping duplication** in `repository/{application,institution,offer}.rs`:
  extract a private `fn row_to_x(row) -> Result<X, ModelError>` per file, called from both
  `find_by_id`/`find_by_*` and `find_all`. Plain function, not a generic — same type, same
  file, zero new abstraction, and it directly fixes a real drift bug already observed (one
  copy of the duplicated code has more `// TODO` comments than the other, i.e. it's already
  diverging).
- **DO NOW — `services/enum_service.rs`'s `build()`**: same disease as the repositories
  above, 9 copy-pasted `if is_empty { create } else { skip }` blocks (confirmed by direct
  read) — apply the same macro/loop treatment.
- **DEFER — `api/*.rs` request boilerplate** (client-build + headers + throttling ticker +
  rate-limit retry loop, duplicated across `offers.rs`/`offers_university.rs`/
  `applications.rs`; `institution.rs` doesn't even need the ticker): this boilerplate is
  small (~10-15 lines) but interleaved with delicate, very different parsing logic per
  file (regex tag extraction, pagination cursors). Extracting a shared `Fetcher`
  abstraction now means redesigning control flow at the same time as fixing the
  `CoreError` gap — two risky changes at once. Also: this is exactly the seam that would
  need to change if EDBO's 2026 behavior forces a different transport — better to leave it
  concrete until that phase actually starts, not guess at the right abstraction now.
- **DEFER — `model/{degree,institution_category,offer_type,ownership_form,priority,
  region,status,study_form}.rs`** (8 enum files): less uniform than the repository
  cluster — `Priority` isn't purely integer-backed, `status.rs` has a redundant hand-rolled
  `TryFrom<i32>` sitting next to a derived `TryFromPrimitive` (looks like a real small bug,
  worth its own separate tiny fix, unrelated to this pass). Forcing these into a
  `speciality.rs`-style macro would need enough escape hatches to be more complex than the
  code it replaces — leave as plain, similar-but-not-identical files.

Do the 3 "do now" items as their own small commits, distinct from the `CoreError`/
schema-drift fixes, even though they land in the same overall pass (CONTRIBUTING.md: one
logical change per commit).

### 5. Wire `server` to call `engine`

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
of `eprintln!` since this call happens after the logger is initialized (the earlier three
calls use `eprintln!` specifically because the logger isn't up yet). `_context` is
discarded for now — nothing downstream consumes scraped data yet (that's a future phase,
same as placement/allocation). Not worth a `From<RuntimeSettings> for EngineConfig` impl
for one field.

### 6. Explicitly out of scope for this pass

- EDBO Cloudflare/anti-bot handling beyond the empirical check already done — validate
  further only once the pipeline runs end-to-end again. If it turns out to be a real
  blocker, the recommended next step (not part of this pass) is a Playwright/headless-
  browser-driven capture of live XHR traffic to learn the new endpoint shape (see
  abit-assistant research above) — **never** a switch to `vstup.osvita.ua` or any
  non-EDBO source; that's off the table by explicit project rule regardless of how EDBO
  scraping goes.
- Placement/allocation algorithm.
- DBML doc fixes (small follow-up, not blocking).

## Verification

1. After each numbered step in section 2: `cargo build --workspace` — confirm no errors
   before moving to the next layer.
2. Once fully wired: `cargo test --package engine crypto::tests` — should run and pass 3
   tests (currently runs 0 because `crypto` isn't `mod`-declared).
3. `cargo clippy --workspace --all-targets` — confirm the pedantic/nursery warnings tied to
   the schema-drift casts are gone, and no new pedantic warnings appear in the newly-live
   code that predates the current lint config.
4. End-to-end: bring up `docker-compose.dev.yaml`'s `db`, run `cargo run --package server`
   against it, and watch how far the real pipeline gets against live EDBO — specifically
   whether `institutions`/`offers`/`offers_university` (GET-based) succeed, confirming the
   Cloudflare non-issue found in recon, and whether the `applications` POST-pagination loop
   (the untested, highest-risk endpoint) succeeds or gets rate-limited/blocked. This result
   directly answers whether a 2026 Cloudflare-bypass phase is even necessary, or whether
   it's just a matter of updated endpoint paths (the registry API 404 found in recon
   suggests the latter is at least partly true).
