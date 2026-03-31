import java.util.List;

/**
 * Befüllt die Supabase-Datenbank mit 3 Beispiel-Mitarbeitern
 * und jeweils ein paar Monaten Lohn-/Zeitdaten.
 * Daten bleiben in der DB stehen.
 */
public class ConnectionTest {

    public static void main(String[] args) {
        System.out.println("============================================");
        System.out.println("  BEISPIELDATEN EINFÜGEN - 3 Mitarbeiter");
        System.out.println("============================================");
        System.out.println();

        SQLConnectionBase sql = new SQLConnectionBase();

        String id1 = "01";
        String id2 = "02";
        String id3 = "03";

        try {
            sql.checkConstTable();

            // Alte Testdaten aufräumen (MA001-MA003 und 01-03)
            System.out.println("Alte Testdaten aufräumen...");
            for (String oldId : new String[]{"MA001","MA002","MA003", id1, id2, id3}) {
                sql.deleteConstContent(oldId);
                sql.dropmtlTable(oldId);
                try (java.sql.Connection con = DatabaseManager.getInstance().getConnection();
                     java.sql.PreparedStatement pst = con.prepareStatement("DELETE FROM allids WHERE ids = ?")) {
                    pst.setString(1, oldId);
                    pst.executeUpdate();
                }
            }
            System.out.println("  [OK] Aufgeräumt");
            System.out.println();

            // =============================================
            // MITARBEITER 1 - Anna Weber, Bürokauffrau
            // =============================================
            System.out.println(">>> Mitarbeiter 1: Anna Weber");
            sql.insertintoConstCalc("arbeitnehmerkonstanten", id1,
                "Anna", "Weber", "Schneider", "Hauptstraße", "12",
                "50667", "Köln", "15.03.1988", "w", "deutsch",
                "P-1001", "12 150388 W 042", "11111", "001",
                "Bürokauffrau", "101", "57 041 382 951", "1",
                "01.04.2022", "3200.00", "7.3", "9.3", "1.7",
                "0.6", "0.12", "14.0", "Müller & Söhne GmbH", "12345678",
                "216/5738/0291", "Industrieweg", "5", "50668", "Köln",
                "Müller & Söhne", "0.00");
            sql.insertintoAllIdsTable(id1);
            sql.createmtlTable(id1);

            // Januar 2025
            sql.insertintomtlCalc(id1, "2025", "01",
                3200.0, 0.0, 2185.60, 3200.0,
                233.60, 297.60, 54.40, 19.20, 3.84, 448.00,
                249.60, 297.60, 41.60, 19.20, 3.84, 0.0,
                1668.48, "31.01.2025");
            sql.updateContent(id1, "arbeitszeit", "01", "2025", "168.0");
            sql.updateContent(id1, "urlaub", "01", "2025", "0");
            sql.updateContent(id1, "krank", "01", "2025", "0");

            // Februar 2025
            sql.insertintomtlCalc(id1, "2025", "02",
                3200.0, 0.0, 2185.60, 3200.0,
                233.60, 297.60, 54.40, 19.20, 3.84, 448.00,
                249.60, 297.60, 41.60, 19.20, 3.84, 0.0,
                1668.48, "28.02.2025");
            sql.updateContent(id1, "arbeitszeit", "02", "2025", "152.0");
            sql.updateContent(id1, "urlaub", "02", "2025", "2");
            sql.updateContent(id1, "krank", "02", "2025", "0");

            // März 2025
            sql.insertintomtlCalc(id1, "2025", "03",
                3200.0, 500.0, 2522.10, 3700.0,
                270.10, 344.10, 62.90, 22.20, 4.44, 518.00,
                288.60, 344.10, 48.10, 22.20, 4.44, 0.0,
                1929.18, "31.03.2025");
            sql.updateContent(id1, "arbeitszeit", "03", "2025", "176.0");
            sql.updateContent(id1, "urlaub", "03", "2025", "0");
            sql.updateContent(id1, "krank", "03", "2025", "1");

            System.out.println("  [OK] Anna Weber angelegt + 3 Monate Daten");
            System.out.println();

            // =============================================
            // MITARBEITER 2 - Tobias Richter, Elektriker
            // =============================================
            System.out.println(">>> Mitarbeiter 2: Tobias Richter");
            sql.insertintoConstCalc("arbeitnehmerkonstanten", id2,
                "Tobias", "Richter", "Richter", "Am Marktplatz", "7a",
                "40210", "Düsseldorf", "22.09.1995", "m", "deutsch",
                "P-1002", "14 220995 R 081", "22222", "003",
                "Elektriker", "101", "21 095 447 382", "1",
                "15.08.2023", "3800.00", "7.3", "9.3", "1.7",
                "0.6", "0.12", "15.5", "Elektro Schmidt KG", "87654321",
                "113/5821/0474", "Werkstraße", "22", "40211", "Düsseldorf",
                "Elektro Schmidt", "32.50");
            sql.insertintoAllIdsTable(id2);
            sql.createmtlTable(id2);

            // Januar 2025
            sql.insertintomtlCalc(id2, "2025", "01",
                3800.0, 0.0, 2608.20, 3800.0,
                277.40, 353.40, 64.60, 22.80, 4.56, 532.00,
                296.40, 353.40, 49.40, 22.80, 4.56, 0.0,
                1981.32, "31.01.2025");
            sql.updateContent(id2, "arbeitszeit", "01", "2025", "172.0");
            sql.updateContent(id2, "arbeitszeitregie", "01", "2025", "16.0");
            sql.updateContent(id2, "urlaub", "01", "2025", "0");
            sql.updateContent(id2, "krank", "01", "2025", "0");

            // Februar 2025
            sql.insertintomtlCalc(id2, "2025", "02",
                3800.0, 0.0, 2608.20, 3800.0,
                277.40, 353.40, 64.60, 22.80, 4.56, 532.00,
                296.40, 353.40, 49.40, 22.80, 4.56, 0.0,
                1981.32, "28.02.2025");
            sql.updateContent(id2, "arbeitszeit", "02", "2025", "160.0");
            sql.updateContent(id2, "arbeitszeitregie", "02", "2025", "24.0");
            sql.updateContent(id2, "urlaub", "02", "2025", "0");
            sql.updateContent(id2, "krank", "02", "2025", "3");

            System.out.println("  [OK] Tobias Richter angelegt + 2 Monate Daten");
            System.out.println();

            // =============================================
            // MITARBEITER 3 - Lena Fischer, Grafikdesignerin
            // =============================================
            System.out.println(">>> Mitarbeiter 3: Lena Fischer");
            sql.insertintoConstCalc("arbeitnehmerkonstanten", id3,
                "Lena", "Fischer", "Braun", "Lindenallee", "33",
                "60311", "Frankfurt", "08.12.1992", "w", "deutsch",
                "P-1003", "16 081292 F 015", "33333", "005",
                "Grafikdesignerin", "101", "84 012 559 738", "1",
                "01.01.2024", "4100.00", "7.3", "9.3", "1.7",
                "0.6", "0.12", "14.0", "Kreativwerk AG", "55443322",
                "045/8912/0638", "Mediaplatz", "10", "60313", "Frankfurt",
                "Kreativwerk", "0.00");
            sql.insertintoAllIdsTable(id3);
            sql.createmtlTable(id3);

            // Januar 2025
            sql.insertintomtlCalc(id3, "2025", "01",
                4100.0, 0.0, 2809.40, 4100.0,
                299.30, 381.30, 69.70, 24.60, 4.92, 574.00,
                319.80, 381.30, 53.30, 24.60, 4.92, 0.0,
                2137.74, "31.01.2025");
            sql.updateContent(id3, "arbeitszeit", "01", "2025", "168.0");
            sql.updateContent(id3, "urlaub", "01", "2025", "0");
            sql.updateContent(id3, "krank", "01", "2025", "0");

            // Februar 2025
            sql.insertintomtlCalc(id3, "2025", "02",
                4100.0, 0.0, 2809.40, 4100.0,
                299.30, 381.30, 69.70, 24.60, 4.92, 574.00,
                319.80, 381.30, 53.30, 24.60, 4.92, 0.0,
                2137.74, "28.02.2025");
            sql.updateContent(id3, "arbeitszeit", "02", "2025", "144.0");
            sql.updateContent(id3, "urlaub", "02", "2025", "3");
            sql.updateContent(id3, "krank", "02", "2025", "0");

            // März 2025
            sql.insertintomtlCalc(id3, "2025", "03",
                4100.0, 1000.0, 3464.40, 5100.0,
                372.30, 474.30, 86.70, 30.60, 6.12, 714.00,
                397.80, 474.30, 66.30, 30.60, 6.12, 0.0,
                2659.14, "31.03.2025");
            sql.updateContent(id3, "arbeitszeit", "03", "2025", "176.0");
            sql.updateContent(id3, "urlaub", "03", "2025", "0");
            sql.updateContent(id3, "krank", "03", "2025", "0");

            System.out.println("  [OK] Lena Fischer angelegt + 3 Monate Daten");
            System.out.println();

            // =============================================
            // ÜBERSICHT
            // =============================================
            System.out.println("============================================");
            System.out.println("  ÜBERSICHT");
            System.out.println("============================================");

            List<String> alleIds = sql.selectAllIds("arbeitnehmerkonstanten");
            int anzahl = sql.countContent("arbeitnehmerkonstanten");
            System.out.println("  Mitarbeiter gesamt: " + anzahl);
            System.out.println("  IDs: " + alleIds);
            System.out.println();

            for (String id : alleIds) {
                String vn = sql.selectConstContent("arbeitnehmerkonstanten", "anvorname", id);
                String nn = sql.selectConstContent("arbeitnehmerkonstanten", "annachname", id);
                String beruf = sql.selectConstContent("arbeitnehmerkonstanten", "anberufsbezeichnung", id);
                String brutto = sql.selectConstContent("arbeitnehmerkonstanten", "anmtlverguetung", id);
                String ag = sql.selectConstContent("arbeitnehmerkonstanten", "agname", id);
                System.out.println("  " + id + ": " + vn + " " + nn + " | " + beruf + " | " + brutto + " EUR | " + ag);
            }

            System.out.println();
            System.out.println("  >>> FERTIG - Daten stehen in Supabase! <<<");

        } catch (Exception e) {
            System.out.println("[FEHLER] " + e.getMessage());
            e.printStackTrace();
        } finally {
            DatabaseManager.getInstance().shutdown();
        }
    }
}

