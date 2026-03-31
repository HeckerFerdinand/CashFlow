import com.zaxxer.hikari.HikariConfig;
import com.zaxxer.hikari.HikariDataSource;

import java.sql.Connection;
import java.sql.SQLException;

/**
 * Singleton-Klasse, die den HikariCP Connection Pool verwaltet.
 * Stellt Datenbankverbindungen effizient über einen Pool bereit,
 * anstatt jedes Mal eine neue Verbindung aufzubauen.
 */
public class DatabaseManager {

    private static volatile DatabaseManager instance;
    private final HikariDataSource dataSource;

    private DatabaseManager() {
        HikariConfig config = new HikariConfig();
        config.setJdbcUrl(DatabaseConfig.DB_URL);
        config.setUsername(DatabaseConfig.DB_USER);
        config.setPassword(DatabaseConfig.DB_PASSWORD);
        config.setDriverClassName("org.postgresql.Driver");

        // Pool-Einstellungen
        config.setMaximumPoolSize(DatabaseConfig.MAX_POOL_SIZE);
        config.setMinimumIdle(DatabaseConfig.MIN_IDLE);
        config.setConnectionTimeout(DatabaseConfig.CONNECTION_TIMEOUT);
        config.setIdleTimeout(DatabaseConfig.IDLE_TIMEOUT);
        config.setMaxLifetime(DatabaseConfig.MAX_LIFETIME);

        // SSL für Supabase (Remote-Verbindung)
        config.addDataSourceProperty("ssl", "true");
        config.addDataSourceProperty("sslmode", "require");

        // Supabase Session Pooler: serverseitige Prepared Statements deaktivieren
        config.addDataSourceProperty("prepareThreshold", "0");

        dataSource = new HikariDataSource(config);
        System.out.println("Datenbankverbindung zum Pool hergestellt: " + DatabaseConfig.DB_HOST);
    }

    /**
     * Thread-sicherer Singleton-Zugriff (Double-Checked Locking)
     */
    public static DatabaseManager getInstance() {
        if (instance == null) {
            synchronized (DatabaseManager.class) {
                if (instance == null) {
                    instance = new DatabaseManager();
                }
            }
        }
        return instance;
    }

    /**
     * Holt eine Connection aus dem Pool.
     * WICHTIG: Connection muss nach Benutzung geschlossen werden (geht zurück in den Pool)!
     * Am besten mit try-with-resources verwenden.
     */
    public Connection getConnection() throws SQLException {
        return dataSource.getConnection();
    }

    /**
     * Fährt den Connection Pool herunter (z.B. beim Beenden der Anwendung).
     */
    public void shutdown() {
        if (dataSource != null && !dataSource.isClosed()) {
            dataSource.close();
            System.out.println("Datenbankverbindung geschlossen.");
        }
    }
}

