import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.fxml.FXMLLoader;
import javafx.scene.Node;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.scene.control.TextField;
import javafx.stage.Stage;

public class ANAN4Controller {


    @FXML
    private TextField antätigkeittextfield4h;
    @FXML
    private TextField anbgrtextfield4h;
    @FXML
    private TextField anpgruppetextfield4h;
    @FXML
    private TextField anstidtextfield4h;
    @FXML
    private TextField angleittextfield4h;
    @FXML
    private TextField anmtlvtextfield4h;
    @FXML
    private TextField ankvtextfield4h;
    @FXML
    private TextField anrvtextfield4h;
    @FXML
    private TextField anu1textfield4h;
    @FXML
    private TextField anu2textfield4h;
    @FXML
    private TextField aninsotextfield4h;
    @FXML
    private TextField ansttextfield4h;
    @FXML
    private TextField rhtextfield4h;

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
    private String anrhv;


    //Klick auf "Weiter"
    public void handleweiterbutton4h(ActionEvent event) {
        try {
            //Werte speichern
            antätigkeit = antätigkeittextfield4h.getText();
            anbgr = anbgrtextfield4h.getText();
            anpgruppet = anpgruppetextfield4h.getText();
            angleit = angleittextfield4h.getText();
            anstid = anstidtextfield4h.getText();
            anmtlv = anmtlvtextfield4h.getText();
            ankv = ankvtextfield4h.getText().replace(",",".");
            double kv = Double.parseDouble(ankv) / 100;
            ankv = String.valueOf(kv);
            anrv = anrvtextfield4h.getText().replace(",",".");
            double rv = Double.parseDouble(anrv) / 100;
            anrv = String.valueOf(rv);
            anu1 = anu1textfield4h.getText().replace(",",".");
            double u1 = Double.parseDouble(anu1) / 100;
            anu1 = String.valueOf(u1);
            anu2 = anu2textfield4h.getText().replace(",",".");
            double u2 = Double.parseDouble(anu2) / 100;
            anu2 = String.valueOf(u2);
            aninso = aninsotextfield4h.getText().replace(",",".");
            double inso = Double.parseDouble(aninso) / 100;
            aninso = String.valueOf(inso);
            anst = ansttextfield4h.getText().replace(",",".");
            double st = Double.parseDouble(anst) / 100;
            anst = String.valueOf(st);
            anrhv = rhtextfield4h.getText();

            // Lade die neue Szene und hole den Controller
            FXMLLoader loader = new FXMLLoader(getClass().getResource("/ANAN5.fxml"));
            Parent root = (Parent) loader.load();

            // Hole den zweiten Controller
            ANAN5Controller thirdcontroller = loader.getController();

            //Übertrage die Werte in den dritten Controller
            thirdcontroller.setAnvorname3h(anvorname3h);
            thirdcontroller.setAnnachnamename3h(annachnamename3h);
            thirdcontroller.setAngeburtsnamename3h(angeburtsnamename3h);
            thirdcontroller.setAnstraße3h(anstraße3h);
            thirdcontroller.setAnhausnummer3h(anhausnummer3h);
            thirdcontroller.setAnpostleitzahl3h(anpostleitzahl3h);
            thirdcontroller.setAnort3h(anort3h);
            thirdcontroller.setAngeburtsdatum3h(angeburtsdatum3h);
            thirdcontroller.setAnstaatssngehörigkeit3h(anstaatssngehörigkeit3h);
            thirdcontroller.setAnpersonalnummer3h(anpersonalnummer3h);
            thirdcontroller.setAnsvnummer3h(ansvnummer3h);
            thirdcontroller.setAngeschlecht(angeschlecht);
            thirdcontroller.setAnberufsbezeichnung3h(anberufsbezeichnung3h);
            thirdcontroller.setAnbeschäftigungsbeginn3h(anbeschäftigungsbeginn3h);
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
        }
        catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "-"
    public void handleklbutton4h(ActionEvent event) {
        try {
            Main.minimizeWindow();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "x"
    public void handleclbutton4h(ActionEvent event) {
        try {
            Main.openClosePopup("/ClosePopup99.fxml");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "Zurück"
    public void handlebackbutton4h(ActionEvent event) {
        try {
            Main.changeScene("/ANAN3.fxml");
        }
        catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Werte von ANAN3 übernehmen
    public void setAnvorname3h(String anvorname3h) {
        this.anvorname3h = anvorname3h;
    }

    public void setAnnachnamename3h(String annachnamename3h) {
        this.annachnamename3h = annachnamename3h;
    }

    public void setAngeburtsnamename3h(String angeburtsnamename3h) {
        this.angeburtsnamename3h = angeburtsnamename3h;
    }

    public void setAnstraße3h(String anstraße3h) {
        this.anstraße3h = anstraße3h;
    }

    public void setAnhausnummer3h(String anhausnummer3h) {
        this.anhausnummer3h = anhausnummer3h;
    }

    public void setAnpostleitzahl3h(String anpostleitzahl3h) {
        this.anpostleitzahl3h = anpostleitzahl3h;
    }

    public void setAnort3h(String anort3h) {
        this.anort3h = anort3h;
    }

    public void setAngeburtsdatum3h(String angeburtsdatum3h) {
        this.angeburtsdatum3h = angeburtsdatum3h;
    }

    public void setAnstaatssngehörigkeit3h(String anstaatssngehörigkeit3h) {
        this.anstaatssngehörigkeit3h = anstaatssngehörigkeit3h;
    }

    public void setAnpersonalnummer3h(String anpersonalnummer3h) {
        this.anpersonalnummer3h = anpersonalnummer3h;
    }

    public void setAnsvnummer3h(String ansvnummer3h) {
        this.ansvnummer3h = ansvnummer3h;
    }

    public void setAnberufsbezeichnung3h(String anberufsbezeichnung3h) {
        this.anberufsbezeichnung3h = anberufsbezeichnung3h;
    }

    public void setAnbeschäftigungsbeginn3h(String anbeschäftigungsbeginn3h) {
        this.anbeschäftigungsbeginn3h = anbeschäftigungsbeginn3h;
    }

    public void setAngeschlecht(String angeschlecht) {
        this.angeschlecht = angeschlecht;
    }
}
