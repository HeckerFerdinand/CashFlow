import javafx.event.ActionEvent;
import javafx.stage.DirectoryChooser;
import java.io.File;


public class Home1Controller {

    SQLConnectionBase connection = new SQLConnectionBase();

    //Klick auf "Lohnbuchhaltung"
    public void handlelbbutton1() {
        try {
            Main.changeScene("/LAZA2.fxml");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "Stammdaten"
    public void handlesdvbutton1() {
        try {
            Main.changeScene("/ABLE2.fxml");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "Update-Protokoll"
    public void handlenbbutton1() {
        try {
            Main.openProtokoll();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "Impressum"
    public void handleipbutton1() {
        try {
            Main.openImpressum();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public void handlenaspfadbutton() {
        try {
            DirectoryChooser directoryChooser = new DirectoryChooser();
            directoryChooser.setTitle("Speicherort für PDFs auswählen (NAS)");

            // Das Fenster öffnen
            // Wenn du Zugriff auf die 'stage' hast, übergib sie hier.
            // null funktioniert meistens auch, öffnet aber ein freies Fenster.
            File selectedDirectory = directoryChooser.showDialog(null);

            if (selectedDirectory != null) {
                String pfad = selectedDirectory.getAbsolutePath();

                // Hier speichern wir den Pfad in die neue Tabelle
                // Ich nenne die Methode mal 'saveSetting', die bauen wir gleich
                connection.saveSetting("pdf_path", pfad);

                System.out.println("NAS-Pfad erfolgreich gespeichert: " + pfad);

                // Optional: Zeige ein kurzes Bestätigungs-Popup
                // CONFIRMPopup confirm = new CONFIRMPopup();
                // confirm.display("NAS_SAVED.fxml");
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "-"
    public void handleklbutton1() {
        try {
            Main.minimizeWindow();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "x"
    public void handleclbutton1(ActionEvent event) {
        try {
            Main.openClosePopup("/ClosePopup99.fxml");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}




