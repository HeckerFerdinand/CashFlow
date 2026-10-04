# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

CashFlow is a German **Minijob payroll** desktop app (Windows + macOS): employer/employee master data, monthly wage entry with flat-rate employer contributions (KV, RV, U1, U2, Insolvenzgeldumlage, Pauschalsteuer), monthly time entry, a yearly progress overview and PDF documents (payslip, yearly wage and time journals). Rust workspace with a Slint UI; data in Supabase (Auth + PostgREST). UI text and user-facing messages are German; code, identifiers and comments are English. `README.md` and `docs/SUPABASE_SETUP.md` are written in German for the owner.

Version 1 was a Java/JavaFX app; it was removed on the `rust-rewrite` branch and lives only in git history (`master`).

## Commands

```sh
cargo run -p cashflow-app                     # run the app
cargo test --workspace                        # all tests
cargo test -p cashflow-core money             # single module / test filter
cargo clippy --workspace --all-targets        # must stay warning-free (CI uses -D warnings)
cargo fmt --all                               # rustfmt.toml: max_width 120

scripts/dev-db.sh start|reset|stop|env        # local Postgres 17 + PostgREST 14 (portable, in .dev/)
eval "$(scripts/dev-db.sh env)"               # enables the integration tests in crates/data/tests
cargo run -p cashflow-app --example e2e       # drives the real UI end-to-end against the local DB
cargo run -p cashflow-app --example screenshots [page…]   # PNGs of every page at 3 sizes → target/screenshots/
cargo test -p cashflow-documents              # renders sample PDFs → target/document-samples/
```

Without `CASHFLOW_TEST_REST_URL` the integration tests print a note and pass. The local stand-in emulates Supabase's `auth` schema (`supabase/tests/supabase_shim.sql`) and two fixed test users (`supabase/tests/test_users.sql`; `…a11d` is unlocked in `app_users`, `…b10cd` is not). Tests sign JWTs with the local secret instead of using Supabase Auth.

## Architecture

- `crates/core` – pure domain logic, no I/O. `money` (German number parsing/formatting, `Decimal` everywhere, half-away-from-zero rounding), `period`, `validate` (field validators incl. SV-Nummer/Steuer-ID check digits), `employee`/`employer` (validated `*Data` + raw string `*Draft` with `get`/`set` by field key + `validate()`), `payroll` (rates, `calculate`, `PayrollDraft`), `time_record`, `settings` (shared defaults), `files` (PDF file names).
- `crates/data` – `Database` = one Supabase connection: session handling with transparent token refresh/retry (`with_token`), a small PostgREST client (`rest.rs`), Auth client (`auth.rs`) and repositories in `repo/` that add methods to `Database`. `error.rs` maps PostgREST/Auth errors to `DataError` whose `Display` is a German user message (e.g. paused free project, schema missing, user not in `app_users`).
- `crates/documents` – Typst templates (`templates/*.typ`, data passed as virtual `data.json`, all values preformatted in `model.rs`), minimal `World` with embedded Carlito fonts and logo.
- `crates/app` – library + `cashflow` binary.
  - `ui/*.slint`: `app.slint` (window, page switch, re-exports globals for Rust, embeds the Carlito font), `theme.slint` (all colors/sizes, derived from the old app's CSS), `controls.slint` (themed `CfButton`, `CfLineEdit`, `CfComboBox`, `CfCheckBox`, `CfSwitch`, `IconButton`, `CfSpinner`), `icons.slint` (Material icon paths), `state.slint` (shared structs/globals), `widgets.slint` (Card, generic `FormView`/`SectionView` built on `FlexboxLayout`, `SelectionBar`, …), `master-detail.slint`, `pages/*.slint` (one `*Page` global + `*View` per page).
  - `src/context.rs`: `App` (window handle, tokio runtime handle, `Database`, `Cache` of employers/employees/defaults, busy indicator, notices, confirm dialog). Background work: `app.run(...)` / `app.run_db(...)` spawn on tokio and deliver results to the UI thread via `slint::spawn_local`.
  - `src/pages/*.rs`: one module per page with `install` (wire callbacks), `refresh` (on navigation) and page state in a `thread_local!`. `pages/mod.rs` handles navigation and unsaved-change prompts.
  - `src/forms.rs`: field specs → `[FormSection]` model. `config.rs` (per-computer TOML config; build-time defaults `CASHFLOW_SUPABASE_URL/KEY`), `keychain.rs` (refresh token in OS credential store), `output.rs` (atomic PDF writes, open file/folder), `selection.rs` (shared employee/month selection of wage and time entry), `update.rs`.

## Rules that are easy to break

- The contribution formula exists twice: `core::payroll::calculate` and the generated columns of `payroll_records` in `supabase/schema.sql`. Keep them identical; `crates/data/tests/local_supabase.rs` cross-checks them.
- Validation rules in `core` must be at least as strict as the CHECK constraints in `schema.sql`, otherwise users get raw database errors.
- Schema changes: bump `EXPECTED_SCHEMA_VERSION` (`crates/data/src/database.rs`) and the version in `app_status()`; ship a migration SQL file.
- New tables need explicit `grant`s and an RLS policy using `public.is_app_user()` (Supabase no longer exposes new tables automatically).
- Don't use `#[serde(flatten)]` on row types: serde_json's `arbitrary_precision` (needed for exact decimals) breaks it.
- Forms: don't rebuild a form model while the user types (it recreates the `LineEdit`s and drops focus). Update only derived models (e.g. the payroll calculation), rebuild the form on load/save.
- Avoid a page-level `FlexboxLayout` whose items contain word-wrapping text and depend on its height: it hit a Slint layout recursion panic (see payroll/time pages, which switch between `HorizontalLayout`/`VerticalLayout` at 900px instead).
- Never run blocking work (network, Typst) on the UI thread; use `app.run*` (`spawn_blocking` for rendering).
- Only the Supabase *publishable* key may be in the app; `ConnectionSettings` rejects `sb_secret_…`.
- Use the `Cf*` controls, not the std-widgets `Button`/`LineEdit`/`ComboBox`/`CheckBox`/`Switch`: the std widgets take the OS accent color (blue) and cannot be re-themed.
- Gradients: two color stops only, and no `drop-shadow` on elements inside `clip: true` containers – the software renderer (screenshots) draws seams/lines otherwise. Give clipped header bands their own corner radii.
- Packaging: `packaging/windows/CashFlow.iss` (Inno Setup, per-user) and `.github/workflows/release.yml` (tag `vX.Y.Z` must equal the Cargo version). Icons come from `cargo run -p cashflow-app --example icons`.
