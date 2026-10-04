# Auslieferung

CashFlow wird für Windows als Installer gebaut – vollautomatisch auf GitHub.

## Einmalig einrichten

1. **Supabase-Verbindung einbauen (empfohlen):** Im GitHub-Repository unter
   *Settings → Secrets and variables → Actions → Variables* anlegen:
   - `SUPABASE_URL` = Project URL (z. B. `https://abcdefghijklmnop.supabase.co`)
   - `SUPABASE_PUBLISHABLE_KEY` = Publishable key (`sb_publishable_…`)

   Beides ist öffentlich unbedenklich (siehe [SUPABASE_SETUP.md](SUPABASE_SETUP.md)); niemals den
   Secret key oder das Datenbank-Passwort eintragen. Ohne diese Variablen tragen die Anwender die
   Verbindung einmalig im Anmeldefenster ein.

2. **Download-Link öffentlich machen:** GitHub-Release-Dateien sind nur öffentlich herunterladbar,
   wenn das Repository öffentlich ist. Zwei Möglichkeiten:
   - Repository öffentlich schalten (*Settings → General → Danger Zone*). **Vorher** alte
     Zugangsdaten aus der Git-Historie unschädlich machen (altes Supabase-Projekt löschen bzw.
     Passwort ändern, Neon-Zugang ändern) – die Historie enthält sie.
   - oder ein separates, öffentliches Repository nur für Releases verwenden.

   Der Update-Hinweis in CashFlow liest ebenfalls das neueste öffentliche Release.

## Neue Version veröffentlichen

1. Versionsnummer in `Cargo.toml` (`[workspace.package] version`) erhöhen und `CHANGELOG.md` ergänzen.
2. Committen, dann einen Tag mit derselben Nummer pushen:
   ```sh
   git tag v2.0.1
   git push origin v2.0.1
   ```
3. Der Workflow *Release* baut `CashFlow-Setup.exe` und eine portable ZIP-Datei und hängt beide an
   das GitHub-Release (Dauer ca. 10–15 Minuten).

Der dauerhafte Download-Link für Anwender lautet danach immer:

<https://github.com/HeckerFerdinand/CashFlow/releases/latest/download/CashFlow-Setup.exe>

Probe-Build ohne Release: *Actions → Release → Run workflow*; die Dateien liegen dann als
„Artifact“ am Workflow-Lauf.

## Was der Installer macht

- Installation pro Benutzer, **ohne Administratorrechte**, nach `%LOCALAPPDATA%\Programs\CashFlow`.
- Startmenü-Eintrag, auf Wunsch Desktop-Verknüpfung, Deinstallation über die Windows-Einstellungen.
- Ein neuer Installer aktualisiert eine vorhandene Installation; Einstellungen und Anmeldung bleiben
  erhalten (sie liegen im Benutzerprofil, die Daten in Supabase).
- Die App ist nicht signiert: Windows SmartScreen zeigt beim ersten Start evtl. „Unbekannter
  Herausgeber“ → *Weitere Informationen → Trotzdem ausführen*. (Abhilfe: Code-Signing-Zertifikat.)
