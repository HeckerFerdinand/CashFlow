/**
 * Zentrale Konfiguration für die Datenbankverbindung.
 * Hier werden alle Verbindungsdaten an EINER Stelle verwaltet.
 */
public class DatabaseConfig {

    // Supabase Session Pooler Verbindungsdaten
    public static final String DB_HOST = "aws-1-eu-west-1.pooler.supabase.com";
    public static final int DB_PORT = 5432;
    public static final String DB_NAME = "postgres";
    public static final String DB_USER = "postgres.nuzhfoyhbwpfikmynhtt";
    public static final String DB_PASSWORD = "***REMOVED***";

    public static final String DB_URL = "jdbc:postgresql://" + DB_HOST + ":" + DB_PORT + "/" + DB_NAME;

    // HikariCP Pool-Einstellungen
    public static final int MAX_POOL_SIZE = 5;
    public static final int MIN_IDLE = 2;
    public static final long CONNECTION_TIMEOUT = 10000; // 10 Sekunden
    public static final long IDLE_TIMEOUT = 300000; // 5 Minuten
    public static final long MAX_LIFETIME = 600000; // 10 Minuten

    private DatabaseConfig() {
        // Utility-Klasse, nicht instanziieren
    }
}

