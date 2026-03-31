-- ============================================================
-- SQL-Skript für Supabase: Einmal im SQL-Editor ausführen!
-- Supabase Dashboard -> SQL Editor -> New Query -> Einfügen -> Run
-- ============================================================

-- Tabelle: arbeitnehmerkonstanten (Stammdaten der Arbeitnehmer)
CREATE TABLE IF NOT EXISTS arbeitnehmerkonstanten (
    id TEXT PRIMARY KEY,
    anvorname TEXT,
    annachname TEXT,
    angeburtsname TEXT,
    anstraße TEXT,
    anhausnummer TEXT,
    anpostleitzahl TEXT,
    anort TEXT,
    angeburtsdatum TEXT,
    angeschlecht TEXT,
    anstaatsangehoerigkeit TEXT,
    anpersonalnummer TEXT,
    ansvnummer TEXT,
    antaetigkeitsschluessel TEXT,
    anbgrschluessel TEXT,
    anberufsbezeichnung TEXT,
    anpersonengruppe TEXT,
    ansteuerid TEXT,
    angleitzone TEXT,
    anbeschaeftigungsbeginn TEXT,
    anmtlverguetung TEXT,
    ankv DOUBLE PRECISION,
    anrv DOUBLE PRECISION,
    anu1 DOUBLE PRECISION,
    anu2 DOUBLE PRECISION,
    aninso DOUBLE PRECISION,
    anst DOUBLE PRECISION,
    agname TEXT,
    agbetriebsnummer TEXT,
    agsteuernummer TEXT,
    agstraße TEXT,
    aghausnummer TEXT,
    agpostleitzahl TEXT,
    agort TEXT,
    agname2 TEXT,
    anregiestundenverguetung TEXT
);

-- Tabelle: allids (ID-Verwaltung)
CREATE TABLE IF NOT EXISTS allids (
    ids TEXT PRIMARY KEY
);

-- HINWEIS: Die monatlichen Tabellen (pro Arbeitnehmer) werden
-- weiterhin dynamisch durch die Anwendung erstellt (createmtlTable).
-- Diese haben das Format: CREATE TABLE "<id>" (monat TEXT, ...)
-- Das funktioniert mit Supabase genau wie vorher mit der lokalen DB.

