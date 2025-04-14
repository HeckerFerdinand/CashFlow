import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.fxml.FXMLLoader;
import javafx.scene.Node;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.scene.control.CheckBox;
import javafx.scene.control.TextField;
import javafx.stage.Stage;


public class ANBE3Controller {

    @FXML
    private TextField anvornametextfield3i;
    @FXML
    private TextField annachnametextfield3i;
    @FXML
    private TextField angeburtsnametextfield3i;
    @FXML
    private TextField anstraßetextfield3i;
    @FXML
    private TextField anhausnummertextfield3i;
    @FXML
    private TextField anpostleitzahltextfield3i;
    @FXML
    private TextField anorttextfield3i;
    @FXML
    private TextField angeburtsdatumtextfield3i;
    @FXML
    private TextField anstaatsangehörigkeittextfield3i;
    @FXML
    private CheckBox anmännlichcheckbox3i;
    @FXML
    private CheckBox anweiblichcheckbox3i;
    @FXML
    private CheckBox andiverscheckbox3i;
    @FXML
    private TextField anpersonalnummertextfield3i;
    @FXML
    private TextField ansvnummertextfield3i;
    @FXML
    private TextField anberufsbezeichnungtextfield3i;
    @FXML
    private TextField anbeschäftigungsbeginntextfield3i;

    private String anvorname3i;
    private String annachnamename3i;
    private String angeburtsnamename3i;
    private String anstraße3i;
    private String anhausnummer3i;
    private String anpostleitzahl3i;
    private String anort3i;
    private String angeburtsdatum3i;
    private String anstaatssngehörigkeit3i;
    private String anpersonalnummer3i;
    private String ansvnummer3i;
    private String anberufsbezeichnung3i;
    private String anbeschäftigungsbeginn3i;
    private String angeschlecht;

    //Klick auf "Weiter"
    public void handleweiterbutton3i(ActionEvent event) {
        try {
            anvorname3i = anvornametextfield3i.getText();
            annachnamename3i = annachnametextfield3i.getText();
            angeburtsnamename3i = angeburtsnametextfield3i.getText();
            angeburtsdatum3i = angeburtsnametextfield3i.getText();
            anstraße3i = anstraßetextfield3i.getText();
            anhausnummer3i = anhausnummertextfield3i.getText();
            anpostleitzahl3i = anpostleitzahltextfield3i.getText();
            anort3i = anorttextfield3i.getText();
            angeburtsdatum3i = angeburtsdatumtextfield3i.getText();
            anstaatssngehörigkeit3i = anstaatsangehörigkeittextfield3i.getText();
            anpersonalnummer3i = anpersonalnummertextfield3i.getText();
            ansvnummer3i = ansvnummertextfield3i.getText();
            anberufsbezeichnung3i = anberufsbezeichnungtextfield3i.getText();
            anbeschäftigungsbeginn3i = anbeschäftigungsbeginntextfield3i.getText();

            // Geschlecht anhand der ausgewählten CheckBox speichern
            if (anmännlichcheckbox3i.isSelected()) {
                angeschlecht = "männlich";
            } else if (anweiblichcheckbox3i.isSelected()) {
                angeschlecht = "weiblich";
            } else if (andiverscheckbox3i.isSelected()) {
                angeschlecht = "divers";
            }

            FXMLLoader loader = new FXMLLoader(getClass().getResource("/ANBE4.fxml"));
            loader.setControllerFactory(param -> StaticSingleController.getANBE4Controller());
            Parent root = loader.load();
            ANBE4Controller secondController = loader.getController();

            //Übertrage Werte in den zweiten Controller
            secondController.setAnvorname3i(anvorname3i);
            secondController.setAnnachnamename3i(annachnamename3i);
            secondController.setAngeburtsnamename3i(angeburtsnamename3i);
            secondController.setAnstraße3i(anstraße3i);
            secondController.setAnhausnummer3i(anhausnummer3i);
            secondController.setAnpostleitzahl3i(anpostleitzahl3i);
            secondController.setAnort3i(anort3i);
            secondController.setAngeburtsdatum3i(angeburtsdatum3i);
            secondController.setAnstaatssngehörigkeit3i(anstaatssngehörigkeit3i);
            secondController.setAnpersonalnummer3i(anpersonalnummer3i);
            secondController.setAnsvnummer3i(ansvnummer3i);
            secondController.setAngeschlecht(angeschlecht);
            secondController.setAnberufsbezeichnung3i(anberufsbezeichnung3i);
            secondController.setAnbeschäftigungsbeginn3i(anbeschäftigungsbeginn3i);


            // Ändere die Szene
            Stage stage = (Stage) ((Node) event.getSource()).getScene().getWindow();
            Scene scene = new Scene(root);
            stage.setScene(scene);
            stage.show();

            //Werte der nächsten Szene setzen
            String anid = ANW99PopupController.anid;
            SQLConnectionBase sqlConnection = new SQLConnectionBase();
            String antät4i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","antaetigkeitsschluessel", anid);
            String anbgr4i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anbgrschluessel", anid);
            String anpgruppe4i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anpersonengruppe", anid);
            String anstid4i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","ansteuerid", anid);
            String angleit4i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","angleitzone", anid);
            String anmtlv4i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anmtlverguetung", anid);
            String anstds4i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anregiestundenverguetung", anid);
            String ankv4i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","ankv", anid);
            double doubleankv4i = Double.parseDouble(ankv4i) * 100;
            ankv4i = String.valueOf(doubleankv4i);
            String anrv4i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anrv", anid);
            double doubleanrv4i = Double.parseDouble(anrv4i) * 100;
            anrv4i = String.valueOf(doubleanrv4i);
            String anu14i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anu1", anid);
            double doubleanu14i = Double.parseDouble(anu14i) * 100;
            anu14i = String.valueOf(doubleanu14i);
            String anu24i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anu2", anid);
            double doubleanu24i = Double.parseDouble(anu24i) * 100;
            anu24i = String.valueOf(doubleanu24i);
            String aninso4i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","aninso", anid);
            double doubleaninso4i = Double.parseDouble(aninso4i) * 100;
            aninso4i = String.valueOf(doubleaninso4i);
            String anst4i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anst", anid);
            double doubleanst4i = Double.parseDouble(anst4i) * 100;
            anst4i = String.valueOf(doubleanst4i);
            Main.setANBE4Content(antät4i, anbgr4i, anpgruppe4i, anstid4i, angleit4i, anmtlv4i, anstds4i, ankv4i, anrv4i, anu14i, anu24i, aninso4i, anst4i);

        }
        catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "-"
    public void handleklbutton3i(ActionEvent event) {
        try {
            Main.minimizeWindow();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "x"
    public void handleclbutton3i(ActionEvent event) {
        try {
            Main.openClosePopup("/ClosePopup99.fxml");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "Zurück"
    public void handlebackbutton3i(ActionEvent event) {
        try {
            Main.changeScene("/ABLE2.fxml");
        }
        catch (Exception e) {
            e.printStackTrace();
        }
    }

    //TextFiels setzen
    public void setanvornametextfield3i(String text) {
        if (anvornametextfield3i != null)
            anvornametextfield3i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setannachnametextfield3i(String text) {
        if (annachnametextfield3i != null)
            annachnametextfield3i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setangeburtsnametextfield3i(String text) {
        if (angeburtsnametextfield3i != null)
            angeburtsnametextfield3i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setanstraßetextfield3i(String text) {
        if (anstraßetextfield3i != null)
            anstraßetextfield3i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setanhausnummertextfield3i(String text) {
        if (anhausnummertextfield3i != null)
            anhausnummertextfield3i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setanpostleitzahltextfield3i(String text) {
        if (anpostleitzahltextfield3i != null)
            anpostleitzahltextfield3i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setanorttextfield3i(String text) {
        if (anorttextfield3i != null)
            anorttextfield3i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setangeburtsdatumtextfield3i(String text) {
        if (angeburtsdatumtextfield3i != null)
            angeburtsdatumtextfield3i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setanstaatsangehörigkeittextfield3i(String text) {
        if (anstaatsangehörigkeittextfield3i != null)
            anstaatsangehörigkeittextfield3i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setanpersonalnummertextfield3i(String text) {
        if (anpersonalnummertextfield3i != null)
            anpersonalnummertextfield3i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setansvnummertextfield3i(String text) {
        if (ansvnummertextfield3i != null)
            ansvnummertextfield3i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setanberufsbezeichnungtextfield3i(String text) {
        if (anberufsbezeichnungtextfield3i != null)
            anberufsbezeichnungtextfield3i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setanbeschäftigungsbeginntextfield3i(String text) {
        if (anbeschäftigungsbeginntextfield3i != null)
            anbeschäftigungsbeginntextfield3i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setanmännlichcheckbox3i() {
            anmännlichcheckbox3i.setSelected(true);
    }

    public void setanweiblichcheckbox3i() {
            anweiblichcheckbox3i.setSelected(true);
    }

    public void setandiverscheckbox3i() {
            andiverscheckbox3i.setSelected(true);
    }
}

