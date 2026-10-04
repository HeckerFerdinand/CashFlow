# CashFlow

Lohnabrechnung für Minijobs – Desktop-App für Windows und macOS. Die Daten liegen in einer
Supabase-Datenbank (PostgreSQL), sodass mehrere Personen und Rechner mit demselben Bestand arbeiten.

- **Stammdaten:** Arbeitgeber und Mitarbeiter; Eingaben werden geprüft (Pflichtfelder, Datumsangaben,
  Beträge, Prüfziffern von SV-Nummer und Steuer-ID, Postleitzahl …).
- **Lohnerfassung:** Brutto-Bezüge, Regiestunden-Vergütung, Auszahlung. Die Arbeitgeberabgaben
  (KV, RV, U1, U2, Insolvenzgeldumlage, Pauschalsteuer) werden live berechnet und von der Datenbank
  auf den Cent genau gespeichert. Vorschläge kommen aus den Stammdaten und der Zeiterfassung.
- **Zeiterfassung:** Arbeitszeit, Regiestunden, Urlaubs- und Krankheitstage.
- **Übersicht:** welche Monate pro Mitarbeiter erfasst sind; ein Klick öffnet die Erfassung.
- **Dokumente (PDF):** Lohnabrechnung, Lohnjournal und Zeitjournal – je Mitarbeiter ein Ordner
  „Personalnummer Nachname“ im frei wählbaren Ablageordner (z. B. Netzlaufwerk).

## Erste Schritte

1. Datenbank einrichten: **[docs/SUPABASE_SETUP.md](docs/SUPABASE_SETUP.md)**
2. CashFlow starten, Verbindung eintragen, anmelden.

## Entwicklung

Voraussetzung: [Rust](https://rustup.rs) (stabil, ≥ 1.88). Unter Linux zusätzlich
`libfontconfig1-dev` und `libxkbcommon-dev`.

```sh
cargo run -p cashflow-app            # App starten
cargo test --workspace               # alle Tests
cargo clippy --workspace --all-targets
```

### Aufbau

| Crate | Inhalt |
|---|---|
| `crates/core` | Fachlogik ohne I/O: Mitarbeiter, Arbeitgeber, Beitragsberechnung, Eingabeprüfung, deutsche Zahlen/Daten |
| `crates/data` | Supabase-Zugriff: Anmeldung (Auth), Tabellen über die Data API (PostgREST), verständliche Fehlermeldungen |
| `crates/documents` | PDFs mit [Typst](https://typst.app); Vorlagen in `crates/documents/templates/*.typ` |
| `crates/app` | Oberfläche mit [Slint](https://slint.dev): `ui/*.slint` (Aussehen), `src/pages/*.rs` (Abläufe je Seite) |
| `supabase/schema.sql` | Datenbankschema inkl. Zugriffsschutz (Row Level Security) |

Die Oberfläche hat keine festen Pixelpositionen: Layouts passen sich der Fenstergröße an,
Formulare brechen auf schmalen Bildschirmen in weniger Spalten um.

### Lokale Testdatenbank

`scripts/dev-db.sh` lädt portable Versionen von PostgreSQL 17 und PostgREST 14 (wie bei Supabase)
nach `.dev/` und startet damit eine lokale Nachbildung – nichts wird systemweit installiert.

```sh
scripts/dev-db.sh start                 # starten (legt die Datenbank aus supabase/schema.sql an)
eval "$(scripts/dev-db.sh env)"         # Umgebungsvariablen für die Tests
cargo test --workspace                  # inkl. Integrationstests gegen die echte Schemadatei
cargo run -p cashflow-app --example e2e          # bedient die echte App von Anlegen bis Löschen
cargo run -p cashflow-app --example screenshots  # alle Seiten in 3 Fenstergrößen → target/screenshots/
scripts/dev-db.sh reset                 # Datenbank neu anlegen
scripts/dev-db.sh stop
```

Beispiel-PDFs: `cargo test -p cashflow-documents` → `target/document-samples/`.

### Vorkonfigurierte Version bauen

Damit Anwender nur noch E-Mail und Passwort eingeben müssen, kann die Verbindung beim Bauen
eingebettet werden (nur der *Publishable key*, niemals der Secret key):

```sh
CASHFLOW_SUPABASE_URL=https://xxxx.supabase.co CASHFLOW_SUPABASE_KEY=sb_publishable_… \
  cargo build --release -p cashflow-app
```

Für Windows baut der GitHub-Workflow `.github/workflows/release.yml` die `CashFlow.exe`, sobald ein
Tag wie `v2.0.0` gepusht wird (Repository-Variablen `SUPABASE_URL` und `SUPABASE_PUBLISHABLE_KEY`).

### Updates veröffentlichen

1. Version in `Cargo.toml` (`workspace.package.version`) und `CHANGELOG.md` anpassen.
2. Tag pushen, Release-Datei anhängen.
3. Die Versionsdatei des Update-Checks (Gist `version.txt`) auf die neue Nummer setzen –
   CashFlow weist dann beim Start auf das Update hin.

## Änderungen gegenüber Version 1 (Java)

- Komplett neu in Rust; Oberfläche passt sich jeder Bildschirmgröße an (vorher fest für 1920×1080).
- Datenbank neu strukturiert: eine Tabelle je Datenart statt einer Tabelle pro Mitarbeiter;
  Arbeitgeber werden einmal angelegt und zugeordnet; Geldbeträge exakt als `numeric` statt `double`.
- Sicherheit: kein Datenbank-Passwort mehr im Programm; Anmeldung pro Benutzer, Zugriff nur für
  freigeschaltete Benutzer.
- Behobene Fehler u. a.: erster Mitarbeiter ließ sich auf leerer Datenbank nicht anlegen;
  „Zurück“ im Assistenten löschte Personaldaten; Monate konnten doppelt gespeichert werden;
  Fehler wurden als „erfolgreich“ gemeldet; falsche Spalten und Summen im Lohn-/Zeitjournal;
  Februar immer mit 28 Tagen; Sonderzeichen fehlten in PDFs; feste Windows-Pfade.
- Lohn- und Zeiterfassung sind unabhängig voneinander; erneutes Speichern überschreibt den Monat.
- Auszahlungsbetrag wird jetzt auch auf der Lohnabrechnung verwendet (Abzüge = Brutto − Auszahlung).
- Microsofts Calibri-Schrift (nicht weitergebbar) durch die kompatible freie Schrift Carlito ersetzt.

## Lizenzhinweise

Slint wird unter der *Slint Royalty-free Desktop License* verwendet (Hinweis auf der Info-Seite),
Carlito unter der SIL Open Font License (`assets/fonts/OFL.txt`).
