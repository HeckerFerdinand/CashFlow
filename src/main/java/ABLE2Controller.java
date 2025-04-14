import javafx.event.ActionEvent;

import java.io.IOException;

public class ABLE2Controller {


    //Klick auf "Anlegen"
    public void handleanbutton2b(ActionEvent event) {
        try {
            Main.changeScene("/ANAN3.fxml");
        }
        catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "Bearbeiten"
    public void handlebebutton2b(ActionEvent event) {
        Main.openANWPopup("/ANW99.fxml", () -> {
            try {
                Main.changeScene("/ANBE3.fxml");
                String anid = ANW99PopupController.anid;
                SQLConnectionBase sqlConnection = new SQLConnectionBase();
                String anvorname3i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anvorname", anid);
                String annachname3i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","annachname", anid);
                String angeburtsname3i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","angeburtsname", anid);
                String anstraße3i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anstraße", anid);
                String anhausnummer3i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anhausnummer", anid);
                String anpostleitzahl3i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anpostleitzahl", anid);
                String anort3i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anort", anid);
                String angeburtsdatum3i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","angeburtsdatum", anid);
                String anstaatssngehörigkeit3i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anstaatsangehoerigkeit", anid);
                String anpersonalnummer3i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anpersonalnummer", anid);
                String ansvnummer3i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","ansvnummer", anid);
                String anberufsbezeichnung3i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anberufsbezeichnung", anid);
                String anbeschäftigungsbeginn3i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anbeschaeftigungsbeginn", anid);
                String angeschlecht = sqlConnection.selectConstContent("arbeitnehmerkonstanten","angeschlecht", anid);
                Main.setANBE3Content(anvorname3i, annachname3i, angeburtsname3i, anstraße3i, anhausnummer3i, anpostleitzahl3i, anort3i, angeburtsdatum3i, anstaatssngehörigkeit3i, anpersonalnummer3i, ansvnummer3i, anberufsbezeichnung3i, anbeschäftigungsbeginn3i, angeschlecht);
            }   catch (IOException e) {
                e.printStackTrace();
            }
        });
    }

    //Klick auf "Löschen"
    public void handleloebutton2b(ActionEvent event) {
         Main.openANWPopup("/ANW99.fxml", () -> {
             try {
                 Main.openANLBPopup("/ANLB99.fxml", () -> {

                 });
             }
             catch (Exception e) {
                 e.printStackTrace();
             }
         });
    }

    //Klick auf "-"
    public void handleklbutton2b(ActionEvent event) {
        try {
            Main.minimizeWindow();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "x"
    public void handleclbutton2b(ActionEvent event) {
        try {
            Main.openClosePopup("/ClosePopup99.fxml");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "Zurück"
    public void handlebackbutton2b() {
        try {
            Main.changeScene("/Home1.fxml");
        }
        catch (Exception e) {
            e.printStackTrace();
        }
    }
}


