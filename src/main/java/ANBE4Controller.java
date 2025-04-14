import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.fxml.FXMLLoader;
import javafx.scene.Node;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.scene.control.TextField;
import javafx.stage.Stage;

public class ANBE4Controller {

    @FXML
    private TextField antätigkeittextfield4i;
    @FXML
    private TextField anbgrtextfield4i;
    @FXML
    private TextField anpgruppetextfield4i;
    @FXML
    private TextField anstidtextfield4i;
    @FXML
    private TextField angleittextfield4i;
    @FXML
    private TextField anmtlvtextfield4i;
    @FXML
    private TextField ankvtextfield4i;
    @FXML
    private TextField anrvtextfield4i;
    @FXML
    private TextField anu1textfield4i;
    @FXML
    private TextField anu2textfield4i;
    @FXML
    private TextField aninsotextfield4i;
    @FXML
    private TextField ansttextfield4i;
    @FXML
    private TextField rhtextfield4i;

    private String antätigkeit;
    private String anbgr;
    private String anpgruppet;
    private String angleit;
    private String anstid;
    private String anmtlv;
    private String ankv;
    private String anrv;
    private String anu1;
    private String anu2;
    private String aninso;
    private String anst;
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
    private String anrhv;


    //Klick auf "Weiter"
    public void handleweiterbutton4i(ActionEvent event) {
        try {
            //Werte speichern
            antätigkeit = antätigkeittextfield4i.getText();
            anbgr = anbgrtextfield4i.getText();
            anpgruppet = anpgruppetextfield4i.getText();
            angleit = angleittextfield4i.getText();
            anstid = anstidtextfield4i.getText();
            anmtlv = anmtlvtextfield4i.getText();
            ankv = ankvtextfield4i.getText();
            double kv = Double.parseDouble(ankv) / 100;
            ankv = String.valueOf(kv);
            anrv = anrvtextfield4i.getText();
            double rv = Double.parseDouble(anrv) / 100;
            anrv = String.valueOf(rv);
            anu1 = anu1textfield4i.getText();
            double u1 = Double.parseDouble(anu1) / 100;
            anu1 = String.valueOf(u1);
            anu2 = anu2textfield4i.getText();
            double u2 = Double.parseDouble(anu2) / 100;
            anu2 = String.valueOf(u2);
            aninso = aninsotextfield4i.getText();
            double inso = Double.parseDouble(aninso) / 100;
            aninso = String.valueOf(inso);
            anst = ansttextfield4i.getText();
            double st = Double.parseDouble(anst) / 100;
            anst = String.valueOf(st);
            anrhv = rhtextfield4i.getText();

            FXMLLoader loader = new FXMLLoader(getClass().getResource("/ANBE5.fxml"));
            loader.setControllerFactory(param -> StaticSingleController.getANBE5Controller());
            Parent root = loader.load();
            ANBE5Controller thirdcontroller = loader.getController();

            // Setze die Werte im zweiten Controller
            thirdcontroller.setAnvorname3i(anvorname3i);
            thirdcontroller.setAnnachnamename3i(annachnamename3i);
            thirdcontroller.setAngeburtsnamename3i(angeburtsnamename3i);
            thirdcontroller.setAnstraße3i(anstraße3i);
            thirdcontroller.setAnhausnummer3i(anhausnummer3i);
            thirdcontroller.setAnpostleitzahl3i(anpostleitzahl3i);
            thirdcontroller.setAnort3i(anort3i);
            thirdcontroller.setAngeburtsdatum3i(angeburtsdatum3i);
            thirdcontroller.setAnstaatssngehörigkeit3i(anstaatssngehörigkeit3i);
            thirdcontroller.setAnpersonalnummer3i(anpersonalnummer3i);
            thirdcontroller.setAnsvnummer3i(ansvnummer3i);
            thirdcontroller.setAngeschlecht(angeschlecht);
            thirdcontroller.setAnberufsbezeichnung3i(anberufsbezeichnung3i);
            thirdcontroller.setAnbeschäftigungsbeginn3i(anbeschäftigungsbeginn3i);
            //neue Überträge
            thirdcontroller.setAntätigkeit(antätigkeit);
            thirdcontroller.setAnbgr(anbgr);
            thirdcontroller.setAnpgruppet(anpgruppet);
            thirdcontroller.setAngleit(angleit);
            thirdcontroller.setAnstid(anstid);
            thirdcontroller.setAnmtlv(anmtlv);
            thirdcontroller.setAnkv(ankv);
            thirdcontroller.setAnrv(anrv);
            thirdcontroller.setAnu1(anu1);
            thirdcontroller.setAnu2(anu2);
            thirdcontroller.setAninso(aninso);
            thirdcontroller.setAnst(anst);
            thirdcontroller.setAnrhv(anrhv);



            // Ändere die Szene
            Stage stage = (Stage) ((Node) event.getSource()).getScene().getWindow();
            Scene scene = new Scene(root);
            stage.setScene(scene);
            stage.show();

            String anid = ANW99PopupController.anid;
            SQLConnectionBase sqlConnection = new SQLConnectionBase();
            String agname5i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","agname", anid);
            String agvertretung5i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","agname2", anid);
            String agbetriebsnr5i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","agbetriebsnummer", anid);
            String agstnr5i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","agsteuernummer", anid);
            String agstraße5i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","agstraße", anid);
            String aghausnummer5i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","aghausnummer", anid);
            String agpostleitzahl5i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","agpostleitzahl", anid);
            String agort5i = sqlConnection.selectConstContent("arbeitnehmerkonstanten","agort", anid);
            Main.setANBE5Content(agname5i, agvertretung5i, agbetriebsnr5i, agstnr5i, agstraße5i, aghausnummer5i, agpostleitzahl5i, agort5i);
        }
        catch (Exception e) {
            e.printStackTrace();
        }
    }


    //Klick auf "-"
    public void handleklbutton4i(ActionEvent event) {
        try {
            Main.minimizeWindow();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "x"
    public void handleclbutton4i(ActionEvent event) {
        try {
            Main.openClosePopup("/ClosePopup99.fxml");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "Zurück"
    public void handlebackbutton4i(ActionEvent event) {
        try {
            Main.changeScene("/ANBE3.fxml");
        }
        catch (Exception e) {
            e.printStackTrace();
        }
    }


    //TextFiels setzen
    public void setantätigkeittextfield4i(String text) {
        if (antätigkeittextfield4i != null)
            antätigkeittextfield4i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setanbgrtextfield4i(String text) {
        if (anbgrtextfield4i != null)
            anbgrtextfield4i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setanpgruppetextfield4i(String text) {
        if (anpgruppetextfield4i != null)
            anpgruppetextfield4i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setanstid(String text) {
        if (anstidtextfield4i != null)
            anstidtextfield4i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setangleit(String text) {
        if (angleittextfield4i != null)
            angleittextfield4i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setanmtlv4i(String text) {
        if (anmtlvtextfield4i != null)
            anmtlvtextfield4i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setanstds4i(String text) {
        if (rhtextfield4i != null)
            rhtextfield4i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setanrv4i(String text) {
        if (anrvtextfield4i != null)
            anrvtextfield4i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setankv4i(String text) {
        if (ankvtextfield4i != null)
            ankvtextfield4i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setanu14i(String text) {
        if (anu1textfield4i != null)
            anu1textfield4i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setanu24i(String text) {
        if (anu2textfield4i != null)
            anu2textfield4i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setaninso4i(String text) {
        if (aninsotextfield4i != null)
            aninsotextfield4i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setanst4i(String text) {
        if (ansttextfield4i != null)
            ansttextfield4i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }


    //Werte von ANBE3 übernehmen
    public void setAnvorname3i(String anvorname3h) {
        this.anvorname3i = anvorname3h;
    }

    public void setAnnachnamename3i(String annachnamename3h) {
        this.annachnamename3i = annachnamename3h;
    }

    public void setAngeburtsnamename3i(String angeburtsnamename3h) {
        this.angeburtsnamename3i = angeburtsnamename3h;
    }

    public void setAnstraße3i(String anstraße3h) {
        this.anstraße3i = anstraße3h;
    }

    public void setAnhausnummer3i(String anhausnummer3h) {
        this.anhausnummer3i = anhausnummer3h;
    }

    public void setAnpostleitzahl3i(String anpostleitzahl3h) {
        this.anpostleitzahl3i = anpostleitzahl3h;
    }

    public void setAnort3i(String anort3h) {
        this.anort3i = anort3h;
    }

    public void setAngeburtsdatum3i(String angeburtsdatum3h) {
        this.angeburtsdatum3i = angeburtsdatum3h;
    }

    public void setAnstaatssngehörigkeit3i(String anstaatssngehörigkeit3h) {
        this.anstaatssngehörigkeit3i = anstaatssngehörigkeit3h;
    }

    public void setAnpersonalnummer3i(String anpersonalnummer3h) {
        this.anpersonalnummer3i = anpersonalnummer3h;
    }

    public void setAnsvnummer3i(String ansvnummer3h) {
        this.ansvnummer3i = ansvnummer3h;
    }

    public void setAnberufsbezeichnung3i(String anberufsbezeichnung3h) {
        this.anberufsbezeichnung3i = anberufsbezeichnung3h;
    }

    public void setAnbeschäftigungsbeginn3i(String anbeschäftigungsbeginn3h) {
        this.anbeschäftigungsbeginn3i = anbeschäftigungsbeginn3h;
    }

    public void setAngeschlecht(String angeschlecht) {
        this.angeschlecht = angeschlecht;
    }
}
