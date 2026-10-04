# Supabase einrichten

Diese Anleitung richtet die Datenbank für CashFlow 2 ein. Du brauchst sie **einmal**.
Dauer: etwa 15 Minuten.

> Alte Daten werden nicht übernommen – CashFlow 2 startet mit einer leeren Datenbank.

---

## 1. Projekt anlegen

1. Auf <https://supabase.com/dashboard> anmelden (kostenloses Konto genügt).
2. **New project** wählen.
3. Ausfüllen:
   - **Name**: z. B. `cashflow`
   - **Database Password**: ein langes Zufallspasswort erzeugen und im Passwort-Manager speichern.
     CashFlow selbst braucht dieses Passwort **nicht** – es ist nur für Notfälle (z. B. eine Sicherung mit `pg_dump`).
   - **Region**: *Central EU (Frankfurt)* – die Lohndaten bleiben damit in der EU.
4. **Create new project** und ein bis zwei Minuten warten.

## 2. Tabellen anlegen

1. Links **SQL Editor** öffnen → **New query**.
2. Den kompletten Inhalt der Datei [`supabase/schema.sql`](../supabase/schema.sql) hineinkopieren.
3. **Run** klicken. Unten erscheint „Success. No rows returned“.

Das Skript legt alle Tabellen, Prüfregeln und Zugriffsrechte an. Es darf gefahrlos
mehrfach ausgeführt werden.

## 3. Registrierung abschalten

Nur Personen, die du selbst anlegst, sollen sich anmelden können:

1. **Authentication** → **Sign In / Providers**.
2. **Allow new users to sign up** ausschalten und speichern.

(Selbst wenn sich jemand registrieren könnte, sähe er keine Daten – siehe Schritt 5. Doppelt hält besser.)

## 4. Benutzer anlegen

Für jede Person, die mit CashFlow arbeitet:

1. **Authentication** → **Users** → **Add user** → **Create new user**.
2. E-Mail-Adresse und ein Passwort eingeben, **Auto Confirm User** anhaken.
3. **Create user**.

Das Passwort kann später in CashFlow unter *Einstellungen → Benutzerkonto* geändert werden.

## 5. Benutzer freischalten

Ein angelegter Benutzer darf erst nach der Freischaltung Daten sehen. Im **SQL Editor**
(E-Mail-Adresse anpassen) ausführen:

```sql
insert into public.app_users (user_id, note)
select id, 'Ferdinand' from auth.users where email = 'name@beispiel.de';
```

Freischaltung wieder entziehen:

```sql
delete from public.app_users
where user_id = (select id from auth.users where email = 'name@beispiel.de');
```

## 6. Verbindungsdaten für CashFlow

1. **Project Settings** → **Data API**: die **Project URL** kopieren
   (sieht aus wie `https://abcdefghijklmnop.supabase.co`).
2. **Project Settings** → **API Keys**: den **Publishable key** kopieren
   (beginnt mit `sb_publishable_`).

> **Niemals** den *Secret key* (`sb_secret_…`) oder das Datenbank-Passwort in CashFlow eintragen.
> Der Publishable key darf öffentlich sein: ohne Anmeldung und Freischaltung kommt damit niemand an Daten.

## 7. CashFlow verbinden

1. CashFlow starten.
2. Auf der Anmeldeseite **Verbindung zur Datenbank** aufklappen.
3. Project URL und Publishable key eintragen → **Verbindung speichern**.
4. Mit E-Mail und Passwort aus Schritt 4 anmelden.

Danach als Erstes unter *Einstellungen* die **Standardwerte für neue Mitarbeiter** prüfen
(vorbelegt mit den gewerblichen Minijob-Sätzen 2026: KV 13 %, RV 15 %, U1 0,8 %, U2 0,22 %,
Insolvenzgeldumlage 0,15 %, Pauschalsteuer 2 %) und den **Ablageordner** für die PDFs wählen.

Tipp: Wer CashFlow an mehrere Rechner verteilt, kann die Verbindung fest einbauen –
siehe [README](../README.md#vorkonfigurierte-version-bauen). Dann entfallen Schritt 7.2 und 7.3.

---

## Wichtig beim kostenlosen Tarif

**Pausierung:** Kostenlose Projekte werden nach **7 Tagen ohne Nutzung pausiert**. CashFlow
meldet dann „Die Datenbank ist gerade nicht erreichbar …“. Lösung: im Supabase-Dashboard das
Projekt öffnen und **Restore project** klicken; nach ein paar Minuten funktioniert alles wieder.
Es gehen dabei keine Daten verloren.

Wer das vermeiden möchte, hat zwei Möglichkeiten:
- den Pro-Tarif von Supabase (kostenpflichtig), oder
- den mitgelieferten Keep-alive (`.github/workflows/keepalive.yml`): Er ruft alle drei Tage die
  harmlose Funktion `ping()` auf. Dazu im GitHub-Repository unter *Settings → Secrets and
  variables → Actions → Variables* die Variablen `SUPABASE_URL` und `SUPABASE_PUBLISHABLE_KEY` anlegen.

**Sicherungen:** Der kostenlose Tarif enthält keine automatischen Backups. Die erstellten PDFs im
Ablageordner sind bereits ein Archiv der Abrechnungen. Für eine vollständige Sicherung der
Datenbank (z. B. monatlich):

```sh
pg_dump "postgresql://postgres.<projekt-ref>:<db-passwort>@aws-1-eu-central-1.pooler.supabase.com:5432/postgres" \
  --schema=public --data-only --file=cashflow-sicherung.sql
```

(Die genaue Verbindungszeichenkette steht im Dashboard unter **Connect** → *Session pooler*.)

## Später: Datenbank-Updates

Neue CashFlow-Versionen, die die Datenbank ändern, bringen eine zusätzliche SQL-Datei mit
(`supabase/migrations/…`). CashFlow meldet nach der Anmeldung, wenn das Schema veraltet ist.
