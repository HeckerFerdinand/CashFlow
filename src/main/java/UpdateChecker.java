import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.net.URL;
import javax.swing.JOptionPane;

public class UpdateChecker {
    private static final String VERSION = "1.1.0";
    private static final String VERSION_URL = "https://gist.githubusercontent.com/HeckerFerdinand/f0e799558d487d925598c8cf560d8cf5/raw/23b64ab7b3553e4f24e7a30b63992a9e3642a530/version.txt";

    public static void check() {
        try {
            URL url = new URL(VERSION_URL);
            BufferedReader br = new BufferedReader(new InputStreamReader(url.openStream()));
            String latestVersion = br.readLine().trim();

            if (!VERSION.equals(latestVersion)) {
                JOptionPane.showMessageDialog(null,
                        "Neue Version (" + latestVersion + ") verfügbar!\n" +
                                "Bitte lade das Update von GitHub herunter.");
            }
        } catch (Exception e) {
            System.out.println("Update-Check fehlgeschlagen: " + e.getMessage());
        }
    }
}