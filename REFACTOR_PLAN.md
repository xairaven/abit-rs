# abit-rs: architecture and build plan

_Status as of 2026-08-19. Living planning document — update it as steps complete or the
approach changes; it's not a historical record. Code examples are illustrative shape, not
verbatim code to paste — the user is writing the implementation by hand._

## Context

`abit-rs` is being rebuilt into a CLI tool to calculate state-funded ("budgetary")
master's-degree scholarship placement for Ukraine's 2026 admissions cycle. Last year's
attempt (`edbo_core`) only got as far as an incomplete scraper/parser — the
placement/allocation algorithm was never finished.

**Current state:** `edbo_core/` at the repo root is a complete, standalone reference copy
of last year's fully-working implementation — **deliberately not a workspace member**,
exists purely to consult and port logic from. The workspace (`Cargo.toml` members) is
`["server", "scraper", "model"]`:
- `scraper` (renamed `edbo_core` → `engine` → `scraper`) — **the `institution` entity is
  fully implemented end-to-end** (fetch → DTO → model → persist/read-back), the first
  complete vertical slice and the template for `offer`/`application`/`applicant`. Layout
  is DDD/by-entity: `institution.rs` (module declarations only) + `institution/`:
  `api.rs` (`InstitutionApi`, HTTP fetch), `dto.rs` (`InstitutionDto` +
  `TryFrom<InstitutionDto> for Institution`), `service.rs` (`InstitutionService<'a>`,
  fetch-or-cache orchestration), `errors.rs` (`InstitutionError`). `database.rs` owns
  `Database::configure` (schema/table existence check + `sqlx::migrate!()`) and a
  `pool()` accessor. `lib.rs`'s `Scraper::process()` calls `configure()` then
  `InstitutionService::new(&self.database).get()`. Depends on `model`.
- `model` — `Institution` (`institution.rs`), `InstitutionCategory`
  (`institution/category.rs`), `OwnershipForm` (`institution/ownership.rs`), `Region`
  (`region.rs`), `schemas.rs` (schema-name constants). All three lookup enums are
  `#[repr(i16)]` + `Copy, Clone` + `strum::EnumString`/`Display` with Ukrainian
  `#[strum(serialize = "...")]` labels matching EDBO's export text exactly. Deps:
  `num_enum`, `strum`/`strum_macros`.
- `placement` (the allocation algorithm) — **not created yet**, planned.
- `server` — `database.rs` owns "ensure DB exists + open pool" (`Database::init`, no
  admin-DB connection needed), `main.rs` builds the pool (correctly bound to `let db =
  ...`) but **still doesn't construct `Scraper`/call `.process()`** — the next concrete
  wiring step, unchanged from before.

`cargo check`/`cargo clippy --all-targets` (workspace `pedantic`+`nursery` included) both
pass with **zero warnings** on `scraper`+`model` as of this writing — verify this stays
true as more entities are added, don't let it regress.

`docs/dbml/schema.dbml` documents the `common`+`scraped` tables; a `placement`-schema dbml
doc doesn't exist yet.

### EDBO 2025→2026 obstacles — two very different situations, confirmed empirically

**Institutions: solved AND implemented.** Old endpoint
`registry.edbo.gov.ua/api/universities/?exp=json` now 404s (confirmed via `curl`). Real
endpoint:
```
GET https://registry.edbo.gov.ua/api/opendata/universities?rg=<region>&ut=<category>&exp=json
```
**Resolved, no longer open questions:**
- `exp=json` works on the new endpoint too (not just `.xlsx`) — confirmed by pulling a
  real export and diffing its shape against `InstitutionDto`. No `calamine`/spreadsheet
  parsing needed after all; plain `reqwest` + `serde_json`, same as any other JSON API.
- `ut` is `InstitutionCategory`'s numeric discriminant, `rg` is `Region`'s — confirmed
  because a `ut=1` export's category column matched exactly one `InstitutionCategory`
  variant (`HigherEducation`) for every single row.
- Getting *all* institutions doesn't need a loop: `scraper` intentionally hardcodes
  `ut=1`+`rg=0` (`Region::Every`, fetches every region in one call) rather than looping
  over all 8 `InstitutionCategory` values, because **master's programs are only offered
  by category-1 institutions** — the other 7 categories (vocational, secondary, etc.) are
  out of scope for this project, not an oversight.
- No Cloudflare challenge, no encryption on this endpoint — confirmed both via `curl` and
  via a real browser network-tab capture of the actual export button.

Still open: whether offers/applications have an equivalent opendata+json export (worth
checking before assuming the harder path below is the only option) — evidence so far
(the `/offers` capture below) suggests no.

**Offers (and presumably applications): a real, confirmed, much harder obstacle.**
Captured live via browser network tab on `vstup.edbo.gov.ua/offers`:
- **Cloudflare Turnstile is genuinely active** on this flow (auto-solved for a real
  browser session, producing a `captcha-token` JWT cookie) — the first case in all this
  research where Cloudflare is a plausible real blocker for a plain HTTP client, unlike
  every GET path and the institutions endpoint, which sailed through clean.
- **This is not a REST/JSON API — it's Next.js Server Actions.** POSTs go straight to the
  page URL (`/offers`), `Content-Type`/`Accept: text/x-component`, with a `next-action:
  <hash>` header identifying which server function to invoke. These hashes are
  content-addressed to a specific frontend build and will change on redeploy — much more
  fragile than a stable REST path.
- **The payload is a real per-session hybrid encryption handshake**, confirmed by
  capturing both directions:
  1. Request 1 (`[]`, no args) → server returns its RSA public key + a session nonce.
  2. Before request 2, the client generates its own ephemeral RSA keypair + a fresh AES
     key: AES-GCM-encrypts the actual payload (`data`/`iv`/`tag`), RSA-encrypts that AES
     key with the *server's* public key (`key`), and sends its own public key
     (`clientPublicKey`) alongside — captured request 2 payload:
     `["/vstup/speciality_offers/", {key, data, iv, tag, clientPublicKey, meta: "$undefined"}, {}, "captcha-token"]`.
  3. The server decrypts the request the same way in reverse, then encrypts *its*
     response with a fresh AES key wrapped using the *client's* public key from step 2 —
     that's the `{key, data, iv, tag}` shape captured as response 2.
  4. Only the browser tab holding the ephemeral private key it generated can decrypt that
     response.
  - **This is fundamentally different from the old system's obfuscation.** The old
    `crypto.rs` scheme (salt derivable from `number`/`prsid`, a fixed constant) could be
    statically reverse-engineered because the "secret" was really just a formula baked
    into `functions.js`. This is a real per-session key exchange — there's no static
    secret sitting in the JS bundle to extract.

**Decision: drive a real headless browser, don't reimplement the crypto.** Let the actual
EDBO JS do the Turnstile pass and the RSA/AES-GCM handshake — it ends up rendering the
decrypted offers as plain HTML in the DOM. Scrape that, not the network traffic. This also
sidesteps the `next-action` hash fragility entirely, since nothing calls those actions
directly except EDBO's own bundle, whatever hash it currently has.

**Recommended crate: `chromiumoxide`** (async, pure Rust, drives Chrome via CDP directly
and can launch/manage the Chrome subprocess itself — no separate `chromedriver` binary to
install and keep version-matched with the browser, unlike WebDriver-based alternatives
like `thirtyfour`/`fantoccini`).

Illustrative shape (selectors are placeholders — inspect the live page in DevTools for
real ones; nothing here has been live-verified, no browser tool was available when this
was researched):
```rust
use chromiumoxide::{Browser, BrowserConfig};

async fn fetch_offers_html() -> Result<String, FetchError> {
    let (browser, mut handler) = Browser::launch(
        BrowserConfig::builder().with_head().build()?,  // headful while developing
    ).await?;
    let handler_task = tokio::spawn(async move { while handler.next().await.is_some() {} });

    let page = browser.new_page("https://vstup.edbo.gov.ua/offers").await?;
    page.wait_for_navigation().await?;

    // Cloudflare needs a moment to auto-resolve — poll, don't fixed-sleep.
    wait_for_selector(&page, "form.search", Duration::from_secs(15)).await?;

    // Drive the real UI like a human would.
    page.find_element("select[name='speciality']").await?.click().await?;
    // ... pick the option, click search ...

    wait_for_selector(&page, ".offers-results", Duration::from_secs(15)).await?;
    let html = page.content().await?;

    browser.close().await?;
    handler_task.await?;
    Ok(html)
}
```

**Practical considerations, not just the happy path:**
1. **Name collision**: crates.io's HTML-parsing crate is *also* called `scraper`. Usable
   as a dependency inside this project's own `scraper` crate (no compile conflict — own
   crate via `crate::`, dependency via its name), but `use scraper::Html;` inside a crate
   called `scraper` reads oddly. Consider `tl`/`html5ever` directly, or accept the friction.
2. **Reuse one browser/session for the whole run** — launching fresh per offer is slow and
   re-triggers Cloudflare every time.
3. **Docker impact is real**: current `Dockerfile` is `debian:bookworm-slim`. Headless
   Chrome needs the actual browser binary + a pile of system libs (fonts, X11 libs even
   headless) — a genuine image-size/build-complexity increase, not just a `Cargo.toml`
   line.
4. **Detecting "Cloudflare cleared"** needs an explicit poll (the `captcha-token` cookie,
   or a DOM element only present once Turnstile resolves) with a timeout + retries —
   "usually instant" isn't "always instant."
5. **This is a second, genuinely different fetch strategy, not a generalization of the
   first.** Institutions stay on plain `reqwest`+`serde_json` — no crypto, no browser.
   Offers (and presumably applications) need the browser-driven path. Don't force both
   into one shared abstraction just because they're both "fetching" — the mechanics are
   completely different, same reasoning that already applies to `institution::api` vs.
   whatever `offer::api`/`application::api` end up needing.

Corroborating research from `abit-assistant`'s (https://github.com/OlexiyOdarchuk/abit-assistant)
`brain/06 — EDBO Research.md` independently confirms the Next.js rewrite. Two things from
that research still apply:
- **Crypto cross-validation (applicant-name decryption, unrelated to the offers handshake
  above):** abit-assistant independently reverse-engineered the same AES-CBC salt formula
  `edbo_core/src/crypto.rs` already uses (`salt = "v" + (number × (7500 − prsid))`, key =
  SHA256(salt)[:32 hex], IV = SHA256("2025")[:16 hex]) via Playwright capture of the
  archived `vstup2025/js/functions.js`. Combined with `crypto.rs`'s own 3 passing
  golden-value tests, port it unchanged when applications-scraping is built.
- **abit-assistant has not built or live-tested EDBO scraping for 2026 itself** — paused
  pending their own campaign launch, battle-tested sources are `vstup.osvita.ua`/
  `abit-poisk.org.ua`, which **this project is explicitly forbidden from using** (hard
  rule: EDBO only, no fallback, ever). Only their methodology transfers — and the
  Playwright/browser-capture technique is exactly what surfaced the opendata endpoint and
  the offers encryption scheme above, done by hand this time.

### Master's admission algorithm — confirmed research

**No quota categories for master's** (unlike bachelor's — confirmed directly by the
project owner; earlier research in this document mistakenly generalized from
bachelor's-focused sources and has been corrected). What does apply, confirmed against
MON's 2026 procedure (наказ №373, 26.02.2026) and several explainers:
- **"Широкий конкурс" (wide competition):** ranking is per speciality nationally, not per
  specific offer at one institution — has been since 2015. Already fully supported by the
  schema: `offer.speciality_code` + `application.offer_id` let you group applications by
  speciality across every institution's offers; no schema change needed, just correct
  grouping in the algorithm.
- **Core ranking**: sort by competitive score (`application.grade`) descending, then
  priority (`application.priority_code`), then break remaining ties using individual grade
  components (`applicant.grade_components` JSONB) — exactly why that column needs to stay
  a structured breakdown, not a single number.
- **Recursive adjustment**: a recommendation is given only for the highest-priority
  application the applicant qualifies for; once given, lower-priority applications are
  cancelled, freeing seats for lower-ranked applicants — repeats until stable.
- **One main admission wave** (not multiple rounds needing a "wave" concept in the
  schema) — rating lists published, one document-confirmation deadline, then leftover
  seats go to contract.
- **Confirmation matters for correctness**: a recommendation isn't a final placement —
  applicants must confirm original documents by a deadline or lose the spot (rolls to the
  next-ranked applicant). Whether EDBO's actual published `application_status` values
  distinguish "recommended" from "confirmed" needs checking once real status values are
  being scraped.

## Crate topology

```
model  (plain domain types — Institution, Offer, Application, Applicant, lookup enums,
        and eventually PlacementResult. Minimal deps: num_enum, strum. NO reqwest/sqlx.)
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
`scraper` produces, but needs none of `scraper`'s other dependencies (HTTP, crypto, and
eventually a headless browser). Cargo compiles a whole crate's dependency graph as a
unit — depending on all of `scraper` just to see its structs would drag all of that in
for no reason.

**Orphan-rule note, corrected from an earlier version of this doc:** `impl
TryFrom<InstitutionDto> for Institution` lives in `scraper` even though `Institution` (the
`Self` type) is defined in the separate `model` crate — and this is **legal**, confirmed by
the fact it compiles cleanly today. An earlier version of this plan incorrectly predicted
this would violate the orphan rule and need to become a free function. That was wrong:
Rust's orphan rule only requires *some* local type to appear somewhere in the impl
header — it doesn't have to be `Self`. Since `InstitutionDto` (local to `scraper`) appears
as `TryFrom`'s generic parameter, `impl TryFrom<InstitutionDto> for Institution` satisfies
the rule even though `Institution` itself is foreign. This pattern is reusable verbatim for
every future entity (`impl TryFrom<OfferDto> for Offer`, etc.) — no free-function
workaround needed anywhere in this codebase.

## Database: one Postgres database, three schemas

Not three databases — Postgres cannot join or foreign-key across separate databases
without extensions (`postgres_fdw`/`dblink`), real operational overhead for something
`placement` would need constantly. Schemas (namespaces within one database) join and FK
freely, need only one connection/pool, and migrations are just SQL — already reflected in
`scraper/migrations/001_setup.sql`:
- **`common`** — reference/lookup tables + `offer`/`offers_institutions` (anything that
  isn't a real person's data). Written by `scraper`.
- **`scraped`** — `applicant`/`application` (the actual PII-bearing scraped input).
  Written by `scraper`; read-only for `placement`.
- **`placement`** — algorithm output (design TBD). Written by `placement`; read by
  `server` to serve to users.

**Why worth the extra verbosity of schema-qualified table names:** schemas are a real
Postgres permission boundary (`GRANT`/`REVOKE` per schema per role). Since `server` is
meant to eventually serve placement results to end users, a role that only serves results
publicly can eventually be granted access to `placement`+`common` only, never `scraped` —
a bug in the results-serving path can't leak raw scraped PII.

**Migration ownership**: `scraper/migrations/` owns `common`+`scraped` (schema-qualified
throughout, including the `offers_institutions.offer_id → offer.id` FK). A future
`placement` crate embeds its own migrations owning just the `placement` schema, with its
own `sqlx::migrate!()` call against the same database — safe as long as migration version
numbers don't collide between the two crates' migration folders (timestamp-based naming
from `sqlx migrate add` avoids this by construction).

## Architecture within `scraper`: by entity, struct+impl per concern, no traits

`edbo_core` used a `Service` trait (`new(&Database)`) wrapping a `Repository` trait
(`new(&Database)`, `is_empty()`), one impl of each per entity — 5 `services/*.rs` + 13
`repository/*.rs` files, plus `EnumService`, a struct holding 9 repository fields with a
`build()` method that's 9 copy-pasted `if is_empty { create } else { skip }` blocks.

**What was actually wrong with that wasn't "a struct with methods" — it was the trait with
zero polymorphic dispatch anywhere, and the cross-entity god-struct.** Every call site
named the concrete type directly; nothing took a `Box<dyn Repository>` or was generic over
`Service`. That diagnosis still holds. What's now settled, based on direct project-owner
preference: the replacement for "trait + separate Service/Repository files" is **one
struct per entity per concern, `impl` blocks, no trait** — not free functions. The project
owner explicitly prefers methods-on-a-struct over bare functions taking the same parameter
repeatedly, for call-site clarity. Don't push back toward plain functions in review; the
struct shape itself was never the problem.

**The settled per-entity template**, using `institution` (the first fully-built one) as
the reference:
```
scraper/src/institution.rs         // pub mod api; pub mod dto; pub mod errors; pub mod service;
scraper/src/institution/
  api.rs      // InstitutionApi — unit struct, associated fns only (HTTP fetch → Vec<Dto>)
  dto.rs      // InstitutionDto (raw EDBO shape) + impl TryFrom<InstitutionDto> for Institution
  service.rs  // InstitutionService<'a> { database: &'a Database } — the fetch-or-cache
              // orchestrator: get() → is_empty() → fetch+insert, or find_all()
  errors.rs   // InstitutionError — one variant per failure point (dto parse, request,
              // each SQL op, inconsistent-dictionary-data on read-back)
```
Composed into the crate-level `ScraperError` one variant per entity
(`ScraperError::Institution(#[from] InstitutionError)`, alongside
`ScraperError::Database(#[from] DbError)`) — not one flat aggregate with generic
Api/Model/Sql variants. Follow this exact 4-file template for `offer`, `application`,
`applicant` as they're built: same shape, same naming pattern
(`<Entity>Api`/`<Entity>Dto`/`<Entity>Service`/`<Entity>Error`).

### Lookup-table seeding — not yet built, but should follow the same convention

`edbo_core`'s 9 "enum-seed" repositories (`degree`, `region`, `study_form`, `offer_type`,
`ownership_form`, `institution_category`, `status`, plus `knowledge_field`/`speciality`
which don't fit the simple shape) are structurally identical: check `is_empty()`, if so
insert every variant of a Rust enum via `strum::IntoEnumIterator`. When this gets built,
it should follow the same struct+impl convention as `institution/service.rs` rather than
reverting to the macro-generated-free-function shape floated in an earlier version of
this plan — e.g. a `LookupService` (or one small service per lookup table, matching the
per-entity template) with methods doing the is-empty-check + seed insert. The
`sqlx::query!` compile-time-checking constraint still applies (literal SQL per table), so
whatever shape this takes, each table's insert still needs its own literal query string —
a `macro_rules!` generating `impl` methods rather than free functions is the way to keep
that without duplicating 9 near-identical blocks by hand, if that ends up worth doing at
9-tables scale. Decide the exact shape when this is actually built, using `institution` as
the baseline convention, not the earlier plain-function sketch.

## Schema state — resolved items (previously tracked as drift, now consistent)

| Item | Resolution |
|---|---|
| `institution.id`/`parent_id` width | Both migration and `model::Institution` use `i16`/`Option<i16>` consistently — an earlier version of this plan incorrectly kept asserting these needed to become `i32`/`INTEGER`; that was wrong and has been retracted. Real EDBO data (checked across a full category export) comfortably fits `i16` regardless. |
| Enum repr (`Region`, `InstitutionCategory`, `OwnershipForm`) | All `#[repr(i16)]` + `Copy, Clone`, matching their `SMALLINT` columns exactly — no casts needed at the `sqlx` boundary. |
| `institution.registration_year` → `registration_date` | Resolved by renaming + widening rather than parsing: the raw EDBO field is a full `DD.MM.YYYY` date string, not a bare year, so the column/model field is `VARCHAR`/`Option<String>` throughout, storing the raw string as-is (no date parsing — nothing downstream needs to filter/sort by founding date, so there's no reason to build that machinery). |
| `offers_institutions.offer_id → offer.id` FK | Present in the current migration — no longer missing. |

**Still pending, not yet relevant** (no `application`/`offer` entity built yet):
`application.grade`: DBML says `float`, migration says `DECIMAL(10,3)` — when this entity
is built, model should be `bigdecimal::BigDecimal` (already matches the SQL exactly),
converting once from the raw DTO's numeric grade at the DTO→model boundary, same pattern
as `institution`'s conversions. DBML doc drift (`grade` type, `priority_id` vs
`priority_code`, `institution_id` vs `university_id`, missing composite-PK annotations) —
update `docs/dbml/schema.dbml` to match the migration (SQL is source of truth) whenever
convenient, non-blocking.

## Wire `server` to call `scraper`

`server/src/main.rs` currently builds the pool (`let db = Database::init(...)`) but
doesn't yet call into `scraper` — unchanged from before, still the next concrete step.
After the existing `Database::init` block:
```rust
use scraper::{Scraper, ScraperError};
// ...
Scraper::new(&db.pool)
    .process()
    .await
    .unwrap_or_else(|error| {
        log::error!("Scraper process failed. {error}");
        std::process::exit(1);
    });
```
Matches the file's existing `unwrap_or_else` + exit(1) pattern; uses `log::error!` instead
of `eprintln!` since this happens after the logger is initialized.

## Explicitly out of scope for now

- Building the `placement` crate itself (algorithm design) — separate future phase.
- Implementing the headless-browser fetch path for offers/applications — decided on
  approach, not yet built.
- **Never** `vstup.osvita.ua` or any non-EDBO source — off the table by explicit project
  rule regardless of how EDBO scraping goes.
- `docs/dbml/schema.dbml` doc fixes and a `placement`-schema dbml doc — small follow-ups.

## Verification

1. After each build step: `cargo check --package scraper --package model` (or
   `--workspace` once `server` is wired) — confirm no errors before moving on. Currently
   clean.
2. `cargo clippy --package scraper --package model --all-targets` — currently zero
   warnings including `pedantic`+`nursery`; keep it that way as `offer`/`application`/
   `applicant` are added following the `institution` template.
3. Once `crypto` is ported: `cargo test --package scraper crypto::tests` — should pass 3
   golden-value tests.
4. End-to-end: bring up `docker-compose.dev.yaml`'s `db`, wire `server` per above, run
   `cargo run --package server` against it, confirm `Database::configure` correctly
   detects/creates the `common`+`scraped` schemas and tables, then confirm
   `InstitutionService::get()` populates `common.institution` on first run and reads it
   back (skipping the HTTP fetch) on a second run. Offers/applications next, once the
   headless-browser path is built.