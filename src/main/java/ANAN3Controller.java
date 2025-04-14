import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.scene.Node;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.scene.control.TextField;
import javafx.scene.control.CheckBox;
import javafx.fxml.FXMLLoader;
import javafx.stage.Stage;

public class ANAN3Controller {


    @FXML
    private TextField anvornametextfield3h;
    @FXML
    private TextField annachnametextfield3h;
    @FXML
    private TextField angeburtsnametextfield3h;
    @FXML
    private TextField anstraßetextfield3h;
    @FXML
    private TextField anhausnummertextfield3h;
    @FXML
    private TextField anpostleitzahltextfield3h;
    @FXML
    private TextField anorttextfield3h;
    @FXML
    private TextField angeburtsdatumtextfield3h;
    @FXML
    private TextField anstaatsangehörigkeittextfield3h;
    @FXML
    private TextField anpersonalnummertextfield3h;
    @FXML
    private TextField ansvnummertextfield3h;
    @FXML
    private TextField anberufsbezeichnungtextfield3h;
    @FXML
    private TextField anbeschäftigungsbeginntextfield3h;
    @FXML
    private CheckBox anmännlichcheckbox3h;
    @FXML
    private CheckBox anweiblichcheckbox3h;
    @FXML
    private CheckBox andiverscheckbox3h;


    private String anvorname3h;
    private String annachnamename3h;
    private String angeburtsnamename3h;
    private String anstraße3h;
    private String anhausnummer3h;
    private String anpostleitzahl3h;
    private String anort3h;
    private String angeburtsdatum3h;
    private String anstaatssngehörigkeit3h;
    private String anpersonalnummer3h;
    private String ansvnummer3h;
    private String anberufsbezeichnung3h;
    private String anbeschäftigungsbeginn3h;
    private String angeschlecht;


    public void handleweiterbutton3h(ActionEvent event) {
    try {
        // Werte aus den Textfeldern und CheckBoxen speichern
        anvorname3h = anvornametextfield3h.getText();
        annachnamename3h = annachnametextfield3h.getText();
        angeburtsnamename3h = angeburtsnametextfield3h.getText();
        angeburtsdatum3h = angeburtsnametextfield3h.getText();
        anstraße3h = anstraßetextfield3h.getText();
        anhausnummer3h = anhausnummertextfield3h.getText();
        anpostleitzahl3h = anpostleitzahltextfield3h.getText();
        anort3h = anorttextfield3h.getText();
        angeburtsdatum3h = angeburtsdatumtextfield3h.getText();
        anstaatssngehörigkeit3h = anstaatsangehörigkeittextfield3h.getText();
        anpersonalnummer3h = anpersonalnummertextfield3h.getText();
        ansvnummer3h = ansvnummertextfield3h.getText();
        anberufsbezeichnung3h = anberufsbezeichnungtextfield3h.getText();
        anbeschäftigungsbeginn3h = anbeschäftigungsbeginntextfield3h.getText();

        // Geschlecht anhand der ausgewählten CheckBox speichern
        if (anmännlichcheckbox3h.isSelected()) {
            angeschlecht = "männlich";
        } else if (anweiblichcheckbox3h.isSelected()) {
            angeschlecht = "weiblich";
        } else if (andiverscheckbox3h.isSelected()) {
            angeschlecht = "divers";
        }

        // Lade die neue Szene und hole den Controller
        FXMLLoader loader = new FXMLLoader(getClass().getResource("/ANAN4.fxml"));
        Parent root = (Parent) loader.load();

        // Hole den zweiten Controller
        ANAN4Controller secondController = loader.getController();

        // Setze die Werte im zweiten Controller
        secondController.setAnvorname3h(anvorname3h);
        secondController.setAnnachnamename3h(annachnamename3h);
        secondController.setAngeburtsnamename3h(angeburtsnamename3h);
        secondController.setAnstraße3h(anstraße3h);
        secondController.setAnhausnummer3h(anhausnummer3h);
        secondController.setAnpostleitzahl3h(anpostleitzahl3h);
        secondController.setAnort3h(anort3h);
        secondController.setAngeburtsdatum3h(angeburtsdatum3h);
        secondController.setAnstaatssngehörigkeit3h(anstaatssngehörigkeit3h);
        secondController.setAnpersonalnummer3h(anpersonalnummer3h);
        secondController.setAnsvnummer3h(ansvnummer3h);
        secondController.setAngeschlecht(angeschlecht);
        secondController.setAnberufsbezeichnung3h(anberufsbezeichnung3h);
        secondController.setAnbeschäftigungsbeginn3h(anbeschäftigungsbeginn3h);

        // Ändere die Szene
        Stage stage = (Stage) ((Node) event.getSource()).getScene().getWindow();
        Scene scene = new Scene(root);
        stage.setScene(scene);
        stage.show();
    } catch (Exception e) {
        e.printStackTrace();
    }
}


    //Klick auf "-"
    public void handleklbutton3h(ActionEvent event) {
        try {
            Main.minimizeWindow();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "x"
    public void handleclbutton3h(ActionEvent event) {
        try {
            Main.openClosePopup("/ClosePopup99.fxml");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "Zurück"
    public void handlebackbutton3h(ActionEvent event) {
        try {
            Main.changeScene("/ABLE2.fxml");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
