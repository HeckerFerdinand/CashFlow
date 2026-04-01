import java.sql.*;
import java.util.ArrayList;
import java.util.List;


/**
 * Refactored: Nutzt jetzt DatabaseManager (Connection Pool) statt jedes Mal neue Verbindungen.
 * Alle Methoden verwenden PreparedStatements gegen SQL-Injection.
 * Methodensignaturen bleiben kompatibel mit den bestehenden Controllern.
 */
public class SQLConnectionBase {

    private final DatabaseManager db = DatabaseManager.getInstance();

    // --- Hilfsmethode: Einzelnen String-Wert abfragen ---
    private String queryString(String sql, Object... params) {
        try (Connection con = db.getConnection();
             PreparedStatement pst = con.prepareStatement(sql)) {
            for (int i = 0; i < params.length; i++) {
                pst.setObject(i + 1, params[i]);
            }
            ResultSet rs = pst.executeQuery();
            if (rs.next()) {
                String val = rs.getString(1);
                return val != null ? val : "";
            }
            return "";
        } catch (SQLException e) {
            e.printStackTrace();
            return "";
        }
    }

    // --- Hilfsmethode: UPDATE/INSERT/DELETE/DDL ausführen ---
    private int executeUpdate(String sql, Object... params) {
        try (Connection con = db.getConnection();
             PreparedStatement pst = con.prepareStatement(sql)) {
            for (int i = 0; i < params.length; i++) {
                pst.setObject(i + 1, params[i]);
            }
            int rows = pst.executeUpdate();
            System.out.println(rows > 0 ? "Updated" : "Not updated");
            return rows;
        } catch (SQLException e) {
            e.printStackTrace();
            return 0;
        }
    }

    // --- Hilfsmethode: Boolean-Check (EXISTS) ---
    private boolean queryExists(String sql, Object... params) {
        try (Connection con = db.getConnection();
             PreparedStatement pst = con.prepareStatement(sql)) {
            for (int i = 0; i < params.length; i++) {
                pst.setObject(i + 1, params[i]);
            }
            ResultSet rs = pst.executeQuery();
            return rs.next() && rs.getBoolean(1);
        } catch (SQLException e) {
            e.printStackTrace();
            return false;
        }
    }


    public String selectConstContent(String table, String content, String id) {
        // content ist ein Spaltenname, kann nicht parametrisiert werden
        String sql = "SELECT " + content + " FROM " + table + " WHERE id = ?";
        return queryString(sql, id);
    }

    public boolean selectlohnexists(String table, String month, String year) {
        String sql = "SELECT COUNT(*) > 0 FROM \"" + table +
                "\" WHERE monat = ? AND jahr = ? AND verguetungbrutto IS NOT NULL";
        return queryExists(sql, month, year);
    }

    public boolean selectzeitexists(String table, String month, String year) {
        String sql = "SELECT COUNT(*) > 0 FROM \"" + table +
                "\" WHERE monat = ? AND jahr = ? AND arbeitszeit IS NOT NULL";
        return queryExists(sql, month, year);
    }

    public String selectmtlContent(String table, String content, String month, String year) {
        String sql = "SELECT " + content + " FROM \"" + table + "\" WHERE monat = ? AND jahr = ?";
        return queryString(sql, month, year);
    }

    public double selectmtldoubleContent(String table, String content, String month, String year) {
        String result = selectmtlContent(table, content, month, year);
        try {
            return Double.parseDouble(result);
        } catch (NumberFormatException e) {
            return 0.0;
        }
    }


    public void updateConstContent(String table, String id, String anvorname, String annachname, String angeburtsname, String anstraße, String anhausnummer, String anpostleitzahl, String anort, String angeburtsdatum, String angeschlecht, String anstaatsangehörigkeit, String anpersonalnummer, String ansvnummer, String antaetigkeitsschluessel, String anbgrschluessel, String anberufsbezeichnung, String anpersonengruppe, String ansteuerid, String angleitzone, String anbeschaeftugungsbeginn, String anmtlverguetung, String stringankv, String stringanrv, String stringanu1, String stringanu2, String stringaninso, String stringanst, String agname, String agbetriebsnummer, String agsteuernummer, String agstraße, String aghausnummer, String agpostleitzahl, String agort, String agname2, String anregiestundenverguetung) {
        double ankv = Double.parseDouble(stringankv);
        double anrv = Double.parseDouble(stringanrv);
        double anu1 = Double.parseDouble(stringanu1);
        double anu2 = Double.parseDouble(stringanu2);
        double aninso = Double.parseDouble(stringaninso);
        double anst = Double.parseDouble(stringanst);

        String sql = "UPDATE " + table + " SET anvorname=?, annachname=?, angeburtsname=?, anstraße=?, anhausnummer=?, anpostleitzahl=?, anort=?, angeburtsdatum=?, angeschlecht=?, anstaatsangehoerigkeit=?, anpersonalnummer=?, ansvnummer=?, antaetigkeitsschluessel=?, anbgrschluessel=?, anberufsbezeichnung=?, anpersonengruppe=?, ansteuerid=?, angleitzone=?, anbeschaeftigungsbeginn=?, anmtlverguetung=?, ankv=?, anrv=?, anu1=?, anu2=?, aninso=?, anst=?, agname=?, agname2=?, agbetriebsnummer=?, agsteuernummer=?, agstraße=?, aghausnummer=?, agpostleitzahl=?, agort=?, anregiestundenverguetung=? WHERE id=?";
        executeUpdate(sql, anvorname, annachname, angeburtsname, anstraße, anhausnummer, anpostleitzahl, anort, angeburtsdatum, angeschlecht, anstaatsangehörigkeit, anpersonalnummer, ansvnummer, antaetigkeitsschluessel, anbgrschluessel, anberufsbezeichnung, anpersonengruppe, ansteuerid, angleitzone, anbeschaeftugungsbeginn, anmtlverguetung, ankv, anrv, anu1, anu2, aninso, anst, agname, agname2, agbetriebsnummer, agsteuernummer, agstraße, aghausnummer, agpostleitzahl, agort, anregiestundenverguetung, id);
    }

    public boolean updateContent(String table, String content, String month, String year, String input) {
        // content ist ein Spaltenname, kann nicht parametrisiert werden
        String sql = "UPDATE \"" + table + "\" SET " + content + " = ? WHERE monat = ? AND jahr = ?";
        // Numerische Werte als Double übergeben (PostgreSQL castet nicht implizit String -> DOUBLE PRECISION)
        Object value;
        try {
            value = Double.parseDouble(input);
        } catch (NumberFormatException e) {
            value = input;
        }
        return executeUpdate(sql, value, month, year) > 0;
    }

    public int countContent(String table) {
        String sql = "SELECT COUNT(*) FROM " + table;
        try (Connection con = db.getConnection();
             PreparedStatement pst = con.prepareStatement(sql)) {
            ResultSet rs = pst.executeQuery();
            if (rs.next()) {
                return rs.getInt(1);
            }
            return 0;
        } catch (SQLException e) {
            e.printStackTrace();
            return 0;
        }
    }

    public void insertintomtlCalc(String tableid, String year, String month, double vergütungbrutto, double sondervergütungbrutto, double auszahlungsbetrag, double gesamtbetragbrutto, double kv, double rv, double u1, double u2, double inso, double st, double kva, double rva, double u1a, double u2a, double insoa, double sta, double gesamtbeitrag, String datum) {
        String sql = "INSERT INTO \"" + tableid + "\" (monat, verguetungbrutto, sonderverguetungbrutto, auszahlungsbetragbrutto, jahr, gesamtbetragbrutto, kv, rv, u1, u2, inso, st, kva, rva, u1a, u2a, insoa, sta, gesamtbeitrag, datum) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)";
        executeUpdate(sql, month, vergütungbrutto, sondervergütungbrutto, auszahlungsbetrag, year, gesamtbetragbrutto, kv, rv, u1, u2, inso, st, kva, rva, u1a, u2a, insoa, sta, gesamtbeitrag, datum);
    }

    public void insertintoAllIdsTable(String newid) {
        String sql = "INSERT INTO allids (ids) VALUES (?)";
        executeUpdate(sql, newid);
    }

    public void insertintoConstCalc(String table, String id, String anvorname, String annachname, String angeburtsname, String anstraße, String anhausnummer, String anpostleitzahl, String anort, String angeburtsdatum, String angeschlecht, String anstaatsangehörigkeit, String anpersonalnummer, String ansvnummer, String antaetigkeitsschluessel, String anbgrschluessel, String anberufsbezeichnung, String anpersonengruppe, String ansteuerid, String angleitzone, String anbeschaeftugungsbeginn, String anmtlverguetung, String ankv, String anrv, String anu1, String anu2, String aninso, String anst, String agname, String agbetriebsnummer, String steuernummer, String agstraße, String aghausnummer, String agpostleitzahl, String agort, String agname2, String anregiestundenverguetung) {
        // Numerische Werte als Double parsen (Spalten sind DOUBLE PRECISION)
        double dAnkv = Double.parseDouble(ankv);
        double dAnrv = Double.parseDouble(anrv);
        double dAnu1 = Double.parseDouble(anu1);
        double dAnu2 = Double.parseDouble(anu2);
        double dAninso = Double.parseDouble(aninso);
        double dAnst = Double.parseDouble(anst);

        String sql = "INSERT INTO \"" + table + "\" VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)";
        executeUpdate(sql, id, anvorname, annachname, angeburtsname, anstraße, anhausnummer, anpostleitzahl, anort, angeburtsdatum, angeschlecht, anstaatsangehörigkeit, anpersonalnummer, ansvnummer, antaetigkeitsschluessel, anbgrschluessel, anberufsbezeichnung, anpersonengruppe, ansteuerid, angleitzone, anbeschaeftugungsbeginn, anmtlverguetung, dAnkv, dAnrv, dAnu1, dAnu2, dAninso, dAnst, agname, agbetriebsnummer, steuernummer, agstraße, aghausnummer, agpostleitzahl, agort, agname2, anregiestundenverguetung);
    }

    public void createmtlTable(String table) {
        String sql = "CREATE TABLE IF NOT EXISTS \"" + table + "\"(monat TEXT, verguetungbrutto DOUBLE PRECISION, sonderverguetungbrutto DOUBLE PRECISION, auszahlungsbetragbrutto DOUBLE PRECISION, jahr TEXT, gesamtbetragbrutto DOUBLE PRECISION, kv DOUBLE PRECISION, rv DOUBLE PRECISION, u1 DOUBLE PRECISION, u2 DOUBLE PRECISION, inso DOUBLE PRECISION, st DOUBLE PRECISION, kva DOUBLE PRECISION, rva DOUBLE PRECISION, u1a DOUBLE PRECISION, u2a DOUBLE PRECISION, insoa DOUBLE PRECISION, sta DOUBLE PRECISION, gesamtbeitrag DOUBLE PRECISION, arbeitszeit DOUBLE PRECISION, arbeitszeitregie DOUBLE PRECISION, arbeitszeituebertrag DOUBLE PRECISION, gesamtarbeitszeit DOUBLE PRECISION, urlaub DOUBLE PRECISION, krank DOUBLE PRECISION, datum TEXT)";
        executeUpdate(sql);
    }

    public void deleteConstContent(String id) {
        String sql = "DELETE FROM arbeitnehmerkonstanten WHERE id = ?";
        executeUpdate(sql, id);
    }

    public void dropmtlTable(String id) {
        String sql = "DROP TABLE IF EXISTS \"" + id + "\"";
        executeUpdate(sql);
    }

    public String selectmaxConstContent(String coloumn, String table) {
        String sql = "SELECT MAX(" + coloumn + ") AS max_value FROM \"" + table + "\"";
        return queryString(sql);
    }

    public List<String> selectAllIds(String tableName) {
        List<String> ids = new ArrayList<>();
        String sql = "SELECT id FROM " + tableName;
        try (Connection con = db.getConnection();
             PreparedStatement pst = con.prepareStatement(sql);
             ResultSet rs = pst.executeQuery()) {
            while (rs.next()) {
                ids.add(rs.getString("id"));
            }
        } catch (SQLException e) {
            e.printStackTrace();
        }
        return ids;
    }

    public void checkConstTable() {
        try (Connection con = db.getConnection();
             Statement st = con.createStatement()) {
            String checkSql = "SELECT table_name FROM information_schema.tables WHERE table_name = 'arbeitnehmerkonstanten'";
            ResultSet rs = st.executeQuery(checkSql);
            if (!rs.next()) {
                String createSql = "CREATE TABLE arbeitnehmerkonstanten (id TEXT PRIMARY KEY, anvorname TEXT, annachname TEXT, angeburtsname TEXT, anstraße TEXT, anhausnummer TEXT, anpostleitzahl TEXT, anort TEXT, angeburtsdatum TEXT, angeschlecht TEXT, anstaatsangehoerigkeit TEXT, anpersonalnummer TEXT, ansvnummer TEXT, antaetigkeitsschluessel TEXT, anbgrschluessel TEXT, anberufsbezeichnung TEXT, anpersonengruppe TEXT, ansteuerid TEXT, angleitzone TEXT, anbeschaeftigungsbeginn TEXT, anmtlverguetung TEXT, ankv DOUBLE PRECISION, anrv DOUBLE PRECISION, anu1 DOUBLE PRECISION, anu2 DOUBLE PRECISION, aninso DOUBLE PRECISION, anst DOUBLE PRECISION, agname TEXT, agbetriebsnummer TEXT, agsteuernummer TEXT, agstraße TEXT, aghausnummer TEXT, agpostleitzahl TEXT, agort TEXT, agname2 TEXT, anregiestundenverguetung TEXT)";
                st.executeUpdate(createSql);
                System.out.println("Tabelle erfolgreich erstellt: arbeitnehmerkonstanten");
            } else {
                System.out.println("Die Tabelle existiert bereits: arbeitnehmerkonstanten");
            }
        } catch (SQLException e) {
            e.printStackTrace();
        }
    }

    public void saveSetting(String key, String value) {
        String sql = "INSERT INTO app_settings (setting_key, setting_value) VALUES (?, ?) " +
                "ON CONFLICT (setting_key) DO UPDATE SET setting_value = EXCLUDED.setting_value";
        executeUpdate(sql, key, value);
    }

    public String getSetting(String key) {
        String sql = "SELECT setting_value FROM app_settings WHERE setting_key = ?";
        return queryString(sql, key);
    }

    public void checkSettingsTable() {
        String sql = "CREATE TABLE IF NOT EXISTS app_settings (" +
                "setting_key TEXT PRIMARY KEY, " +
                "setting_value TEXT)";
        executeUpdate(sql);
        System.out.println("Tabelle app_settings geprüft/erstellt.");
    }
}
