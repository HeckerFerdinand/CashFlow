import java.sql.*;
import java.util.ArrayList;
import java.util.List;


public class SQLConnectionBase {


    String selectoutput;
    int countoutput;
    String maxoutput;
    boolean updated;


    public String selectConstContent(String table, String content, String id) {
        String url = "jdbc:postgresql://localhost:5432/postgres";
        String username = "postgres";
        String password = "0000";
        try {
            String command = "SELECT " + content + " FROM " + table + " WHERE id = '" + id + "';";
            Class.forName("org.postgresql.Driver");
            Connection con = DriverManager.getConnection(url, username, password);
            Statement st = con.createStatement();
            ResultSet rs = st.executeQuery(command);
            if (rs.next()) {
                selectoutput = rs.getString(1);
            }
            else {
                selectoutput = "";
            }
            con.close();
        }
        catch (SQLException | ClassNotFoundException exception) {
            exception.printStackTrace();
        }
        finally {
            return selectoutput;
        }
    }

    public boolean selectlohnexists(String table, String month, String year) {
        String url = "jdbc:postgresql://localhost:5432/postgres";
        String username = "postgres";
        String password = "0000";
        boolean result = false;

        String sql = "SELECT COUNT(*) > 0 AS eintrag_existiert FROM \"" + table +
                "\" WHERE monat = ? AND jahr = ? AND verguetungbrutto IS NOT NULL";

        try {
            Class.forName("org.postgresql.Driver");
            Connection con = DriverManager.getConnection(url, username, password);

            PreparedStatement pst = con.prepareStatement(sql);
            pst.setString(1, month);
            pst.setString(2, year);

            ResultSet rs = pst.executeQuery();
            if (rs.next()) {
                result = rs.getBoolean("eintrag_existiert");
            }

            con.close();
        } catch (SQLException | ClassNotFoundException e) {
            e.printStackTrace();
        }
        return result;
    }

    public boolean selectzeitexists(String table, String month, String year) {
        String url = "jdbc:postgresql://localhost:5432/postgres";
        String username = "postgres";
        String password = "0000";
        boolean result = false;

        String sql = "SELECT COUNT(*) > 0 AS eintrag_existiert FROM \"" + table +
                "\" WHERE monat = ? AND jahr = ? AND arbeitszeit IS NOT NULL";

        try {
            Class.forName("org.postgresql.Driver");
            Connection con = DriverManager.getConnection(url, username, password);

            PreparedStatement pst = con.prepareStatement(sql);
            pst.setString(1, month);
            pst.setString(2, year);

            ResultSet rs = pst.executeQuery();
            if (rs.next()) {
                result = rs.getBoolean("eintrag_existiert");
            }

            con.close();
        } catch (SQLException | ClassNotFoundException e) {
            e.printStackTrace();
        }

        return result;
    }


    public String selectmtlContent(String table, String content, String month, String year) {
        String url = "jdbc:postgresql://localhost:5432/postgres";
        String username = "postgres";
        String password = "0000";
        try {
            String command = "SELECT " + content + " FROM \"" + table + "\" WHERE monat = '" + month + "' AND jahr = '" + year + "';";
            Class.forName("org.postgresql.Driver");
            Connection con = DriverManager.getConnection(url, username, password);
            Statement st = con.createStatement();
            ResultSet rs = st.executeQuery(command);
            if (rs.next()) {
                selectoutput = rs.getString(1);
            }
            else {
                selectoutput = "";
            }
            con.close();
        }
        catch (SQLException | ClassNotFoundException exception) {
            exception.printStackTrace();
        }
        finally {
            return selectoutput;
        }
    }

    public double selectmtldoubleContent(String table, String content, String month, String year) {
        String url = "jdbc:postgresql://localhost:5432/postgres";
        String username = "postgres";
        String password = "0000";
        try {
            String command = "SELECT " + content + " FROM \"" + table + "\" WHERE monat = '" + month + "' AND jahr = '" + year + "';";
            Class.forName("org.postgresql.Driver");
            Connection con = DriverManager.getConnection(url, username, password);
            Statement st = con.createStatement();
            ResultSet rs = st.executeQuery(command);
            if (rs.next()) {
                selectoutput = rs.getString(1);
            }
            else {
                selectoutput = "";
            }
            con.close();
        }
        catch (SQLException | ClassNotFoundException exception) {
            exception.printStackTrace();
        }
        finally {
            return Double.parseDouble(selectoutput);
        }
    }


    public void updateConstContent(String table, String id, String anvorname, String annachname, String angeburtsname, String anstraße, String anhausnummer, String anpostleitzahl, String anort, String angeburtsdatum, String angeschlecht, String anstaatsangehörigkeit, String anpersonalnummer, String ansvnummer, String antaetigkeitsschluessel, String anbgrschluessel, String anberufsbezeichnung, String anpersonengruppe, String ansteuerid, String angleitzone, String anbeschaeftugungsbeginn, String anmtlverguetung, String stringankv, String stringanrv, String stringanu1, String stringanu2, String stringaninso, String stringanst, String agname, String agbetriebsnummer, String agsteuernummer, String agstraße, String aghausnummer, String agpostleitzahl, String agort, String agname2, String anregiestundenverguetung) {
        String url = "jdbc:postgresql://localhost:5432/postgres";
        String username = "postgres";
        String password = "0000";
        double ankv = Double.parseDouble(stringankv);
        double anrv = Double.parseDouble(stringanrv);
        double anu1 = Double.parseDouble(stringanu1);
        double anu2 = Double.parseDouble(stringanu2);
        double aninso = Double.parseDouble(stringaninso);
        double anst = Double.parseDouble(stringanst);

        String content = "anvorname = '" + anvorname + "', annachname = '" + annachname + "', angeburtsname = '" + angeburtsname + "', anstraße ='" + anstraße + "', anhausnummer = '" + anhausnummer + "', anpostleitzahl = '" + anpostleitzahl + "', anort = '" + anort + "', angeburtsdatum = '" + angeburtsdatum + "', angeschlecht = '" + angeschlecht + "', anstaatsangehoerigkeit = '" + anstaatsangehörigkeit + "', anpersonalnummer = '" + anpersonalnummer + "', ansvnummer = '" + ansvnummer + "', antaetigkeitsschluessel = '" + antaetigkeitsschluessel + "', anbgrschluessel = '" + anbgrschluessel + "', anberufsbezeichnung = '" + anberufsbezeichnung + "', anpersonengruppe = '" + anpersonengruppe + "', ansteuerid = '" + ansteuerid + "', angleitzone = '" + angleitzone + "', anbeschaeftigungsbeginn = '" + anbeschaeftugungsbeginn + "', anmtlverguetung = '" + anmtlverguetung + "', ankv = '" + ankv + "', anrv = '" + anrv + "', anu1 = '" + anu1 + "', anu2 = '" + anu2 + "', aninso = '" + aninso + "', anst = '" + anst + "', agname = '" + agname + "', agname2 = '" + agname2 + "', agbetriebsnummer = '" + agbetriebsnummer + "', agsteuernummer = '" + agsteuernummer + "', agstraße = '" + agstraße + "', aghausnummer = '" + aghausnummer + "', agpostleitzahl = '" + agpostleitzahl + "', agort = '" + agort + "', anregiestundenverguetung = '" + anregiestundenverguetung + "'";
        try{
            String command = "UPDATE " + table + " SET " + content + " WHERE id = '" + id + "';";
            Class.forName("org.postgresql.Driver");
            Connection con = DriverManager.getConnection(url, username, password);
            PreparedStatement preparedStatement = con.prepareStatement(command);
            int affectedrows = preparedStatement.executeUpdate();
            if (affectedrows > 0) {
                System.out.println("Updated");
            }
            else {
                System.out.println("Not updated");
            }
            con.close();
        }
        catch (Exception exception) {
            exception.printStackTrace();
        }
    }

    public boolean updateContent(String table, String content, String month, String year, String input) {
        String url = "jdbc:postgresql://localhost:5432/postgres";
        String username = "postgres";
        String password = "0000";
        try {
            String command = "UPDATE \"" + table + "\" SET " + content + " = '" + input + "'" + " WHERE monat = '" + month + "'" + " AND jahr = '" + year + "';";
            Class.forName("org.postgresql.Driver");
            Connection con = DriverManager.getConnection(url, username, password);
            PreparedStatement preparedStatement = con.prepareStatement(command);
            int affectedrows = preparedStatement.executeUpdate();
            if (affectedrows > 0) {
                updated = true;
            }
            else {
                updated = false;
            }
            con.close();

        }
        catch (SQLException | ClassNotFoundException exception) {
            exception.printStackTrace();
        }
        finally {
            return updated;
        }
    }

    public int countContent(String table) {
        String url = "jdbc:postgresql://localhost:5432/postgres";
        String username = "postgres";
        String password = "0000";
        try {
            String command = "SELECT COUNT(*) FROM " + table ;
            Class.forName("org.postgresql.Driver");
            Connection con = DriverManager.getConnection(url, username, password);
            Statement st = con.createStatement();
            ResultSet rs = st.executeQuery(command);
            rs.next();
            countoutput = rs.getInt(1);
            con.close();

        }
        catch (SQLException | ClassNotFoundException exception) {
            exception.printStackTrace();
        }
        finally {
            return countoutput;
        }
    }

    public void insertintomtlCalc(String tableid, String year, String month,double vergütungbrutto, double sondervergütungbrutto, double auszahlungsbetrag, double gesamtbetragbrutto, double kv, double rv, double u1, double u2, double inso, double st, double kva, double rva, double u1a, double u2a, double insoa, double sta, double gesamtbeitrag, String datum) {
        String url = "jdbc:postgresql://localhost:5432/postgres";
        String username = "postgres";
        String password = "0000";
        try {
            String table = tableid;
            String fullinput = "'" + month + "','" + vergütungbrutto + "','" + sondervergütungbrutto + "','" + auszahlungsbetrag + "','" + year + "','" + gesamtbetragbrutto + "','" + kv + "','" + rv + "','" + u1 + "','" + u2 + "','" + inso + "','" + st + "','" + kva + "','" + rva + "','" + u1a + "','" + u2a + "','" + insoa + "','" + sta + "','" + gesamtbeitrag + "','" + datum + "'";
            String command = "INSERT INTO \"" + table + "\" (monat, verguetungbrutto, sonderverguetungbrutto, auszahlungsbetragbrutto, jahr, gesamtbetragbrutto, kv, rv, u1, u2, inso, st, kva, rva, u1a, u2a, insoa, sta, gesamtbeitrag, datum) VALUES (" + fullinput + ");";
            System.out.println(command);
            Class.forName("org.postgresql.Driver");
            Connection con = DriverManager.getConnection(url, username, password);
            PreparedStatement preparedStatement = con.prepareStatement(command);
            int affectedrows = preparedStatement.executeUpdate();
            if (affectedrows > 0) {
                System.out.println("Updated");
            }
            else {
                System.out.println("Not updated");
            }
            con.close();
        } catch (SQLException | ClassNotFoundException exception) {
            exception.printStackTrace();
        }
    }

    public void insertintoAllIdsTable(String newid){
        String url = "jdbc:postgresql://localhost:5432/postgres";
        String username = "postgres";
        String password = "0000";
        try {

            String command = "INSERT INTO \"allids\" (ids) VALUES (" + newid + ");";
            System.out.println(command);
            Class.forName("org.postgresql.Driver");
            Connection con = DriverManager.getConnection(url, username, password);
            PreparedStatement preparedStatement = con.prepareStatement(command);
            int affectedrows = preparedStatement.executeUpdate();
            if (affectedrows > 0) {
                System.out.println("Updated");
            }
            else {
                System.out.println("Not updated");
            }
            con.close();
        } catch (SQLException | ClassNotFoundException exception) {
            exception.printStackTrace();
        }
    }

    public void insertintoConstCalc(String table, String id, String anvorname, String annachname, String angeburtsname, String anstraße, String anhausnummer, String anpostleitzahl, String anort, String angeburtsdatum, String angeschlecht, String anstaatsangehörigkeit, String anpersonalnummer, String ansvnummer, String antaetigkeitsschluessel, String anbgrschluessel, String anberufsbezeichnung, String anpersonengruppe, String ansteuerid, String angleitzone, String anbeschaeftugungsbeginn, String anmtlverguetung, String ankv, String anrv, String anu1, String anu2, String aninso, String anst, String agname, String agbetriebsnummer, String steuernummer, String agstraße, String aghausnummer, String agpostleitzahl, String agort, String agname2, String anregiestundenverguetung) {
        String url = "jdbc:postgresql://localhost:5432/postgres";
        String username = "postgres";
        String password = "0000";
        try {
            //checkConstTable();
            String fullinput = "'"  + id +  "','" + anvorname + "','" + annachname + "','" +angeburtsname + "','" +anstraße + "','" +anhausnummer + "','" +anpostleitzahl + "','" +anort + "','" +angeburtsdatum + "','" +angeschlecht + "','" +anstaatsangehörigkeit + "','" +anpersonalnummer + "','" +ansvnummer + "','" +antaetigkeitsschluessel + "','" +anbgrschluessel + "','" +anberufsbezeichnung + "','" +anpersonengruppe + "','" +ansteuerid + "','" +angleitzone + "','" +anbeschaeftugungsbeginn + "','" +anmtlverguetung + "','" +ankv + "','" +anrv + "','" +anu1 + "','" +anu2 + "','" +aninso + "','" +anst + "','" +agname + "','" +agbetriebsnummer + "','" +steuernummer+ "','" +agstraße + "','" +aghausnummer + "','" +agpostleitzahl + "','" +agort + "','" +agname2 + "','" +anregiestundenverguetung + "'";
            String command = "INSERT INTO \"" + table + "\"  VALUES (" + fullinput + ");";
            Class.forName("org.postgresql.Driver");
            Connection con = DriverManager.getConnection(url, username, password);
            PreparedStatement preparedStatement = con.prepareStatement(command);
            int affectedrows = preparedStatement.executeUpdate();
            if (affectedrows > 0) {
                System.out.println("Updated");
            }
            else {
                System.out.println("Not updated");
            }
            con.close();
        } catch (SQLException | ClassNotFoundException exception) {
            exception.printStackTrace();
        }
    }

    public void createmtlTable(String table){
        String url = "jdbc:postgresql://localhost:5432/postgres";
        String username = "postgres";
        String password = "0000";
        try {

            String command = "CREATE TABLE \"" + table + "\"(monat TEXT, verguetungbrutto DOUBLE PRECISION, sonderverguetungbrutto DOUBLE PRECISION, auszahlungsbetragbrutto DOUBLE PRECISION, jahr TEXT, gesamtbetragbrutto DOUBLE PRECISION, kv DOUBLE PRECISION, rv DOUBLE PRECISION, u1 DOUBLE PRECISION, u2 DOUBLE PRECISION, inso DOUBLE PRECISION, st DOUBLE PRECISION, kva DOUBLE PRECISION, rva DOUBLE PRECISION, u1a DOUBLE PRECISION, u2a DOUBLE PRECISION, insoa DOUBLE PRECISION, sta DOUBLE PRECISION, gesamtbeitrag DOUBLE PRECISION, arbeitszeit DOUBLE PRECISION, arbeitszeitregie DOUBLE PRECISION, arbeitszeituebertrag DOUBLE PRECISION, gesamtarbeitszeit DOUBLE PRECISION, urlaub DOUBLE PRECISION, krank DOUBLE PRECISION, datum TEXT)";
            Class.forName("org.postgresql.Driver");
            Connection con = DriverManager.getConnection(url, username, password);
            PreparedStatement preparedStatement = con.prepareStatement(command);
            int affectedrows = preparedStatement.executeUpdate();
            if (affectedrows > 0) {
                System.out.println("Updated");
            }
            else {
                System.out.println("Not updated");
            }
            con.close();
        } catch (SQLException | ClassNotFoundException exception) {
            exception.printStackTrace();
        }
    }

    public void deleteConstContent(String id){
        String url = "jdbc:postgresql://localhost:5432/postgres";
        String username = "postgres";
        String password = "0000";
        try {
            String command = "DELETE FROM \"arbeitnehmerkonstanten\" WHERE id = '" + id + "'";
            Class.forName("org.postgresql.Driver");
            Connection con = DriverManager.getConnection(url, username, password);
            PreparedStatement preparedStatement = con.prepareStatement(command);
            int affectedrows = preparedStatement.executeUpdate();
            if (affectedrows > 0) {
                System.out.println("Updated");
            }
            else {
                System.out.println("Not updated");
            }
            con.close();
        } catch (SQLException | ClassNotFoundException exception) {
            exception.printStackTrace();
        }
    }

    public void dropmtlTable(String id){
        String url = "jdbc:postgresql://localhost:5432/postgres";
        String username = "postgres";
        String password = "0000";
        try {
            String command = "DROP TABLE \"" + id + "\"";
            Class.forName("org.postgresql.Driver");
            Connection con = DriverManager.getConnection(url, username, password);
            PreparedStatement preparedStatement = con.prepareStatement(command);
            int affectedrows = preparedStatement.executeUpdate();
            if (affectedrows > 0) {
                System.out.println("Updated");
            }
            else {
                System.out.println("Not updated");
            }
            con.close();
        } catch (SQLException | ClassNotFoundException exception) {
            exception.printStackTrace();
        }
    }

    public String selectmaxConstContent(String coloumn, String table){
        String url = "jdbc:postgresql://localhost:5432/postgres";
        String username = "postgres";
        String password = "0000";

        try {
            String command = "SELECT MAX(" + coloumn + ") AS max_value FROM \"" + table + "\";";
            Class.forName("org.postgresql.Driver");
            Connection con = DriverManager.getConnection(url, username, password);
            Statement st = con.createStatement();
            ResultSet rs = st.executeQuery(command);
            if (rs.next()) {
                maxoutput = rs.getString(1);
            } else {
                maxoutput = "";
            }
            con.close();
        } catch (SQLException | ClassNotFoundException exception) {
            exception.printStackTrace();
        } finally {
            return maxoutput;
        }
    }

    public List<String> selectAllIds(String tableName) {
        String url = "jdbc:postgresql://localhost:5432/postgres";
        String username = "postgres";
        String password = "0000";
        List<String> ids = new ArrayList<>();
        String query = "SELECT id FROM " + tableName;
        try
                (Connection conn = DriverManager.getConnection(url, username , password);
                PreparedStatement pstmt = conn.prepareStatement(query);
                ResultSet rs = pstmt.executeQuery()) {
        while (rs.next()) {
            ids.add(rs.getString("id"));
        }
    }
        catch (Exception e) {
            e.printStackTrace();
        }
        return ids;
    }

    public void checkConstTable() {
        String url = "jdbc:postgresql://localhost:5432/postgres";
        String username = "postgres";
        String password = "0000";
        try (Connection connection = DriverManager.getConnection(url, username, password);
             Statement statement = connection.createStatement()) {

            // Überprüfen, ob die Tabelle existiert
            String checkTableQuery = "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = 'arbeitnehmerkonstanten';";
            ResultSet resultSet = statement.executeQuery(checkTableQuery);

            if (!resultSet.next()) {
                // Tabelle existiert nicht, also wird sie erstellt
                String createTableQuery = "CREATE TABLE arbeitnehmerkonstanten (id TEXT, anvorname TEXT, annachname TEXT, angeburtsname TEXT, anstraße TEXT, anhausnummer TEXT, anpostleitzahl TEXT, anort TEXT, angeburtsdatum TEXT, angeschlecht TEXT, anstaatsangehoerigkeit TEXT, anpersonalnummer TEXT, ansvnummer TEXT, antaetigkeitsschluessel TEXT, anbgrschluessel TEXT, anberufsbezeichnung TEXT, anpersonengruppe TEXT, ansteuerid TEXT, angleitzone TEXT, anbeschaeftigungsbeginn TEXT, anmtlverguetung TEXT, ankv DOUBLE PRECISION, anrv DOUBLE PRECISION, anu1 DOUBLE PRECISION, anu2 DOUBLE PRECISION, aninso DOUBLE PRECISION,anst DOUBLE PRECISION,agname TEXT, agbetriebsnummer TEXT, agsteuernummer TEXT, agstraße TEXT, aghausnummer TEXT, agpostleitzahl TEXT, agort TEXT, agname2 TEXT, anregiestundenverguetung TEXT)";
                statement.executeUpdate(createTableQuery);
                System.out.println("Tabelle erfolgreich erstellt: arbeitnehmerkonstanten");
            } else {
                System.out.println("Die Tabelle existiert bereits: arbeitnehmerkonstanten");
            }
        } catch (SQLException e) {
            e.printStackTrace();
        }
    }
}

