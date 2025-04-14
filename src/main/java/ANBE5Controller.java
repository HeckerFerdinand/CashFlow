import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.scene.control.TextField;

public class ANBE5Controller {

    @FXML
    private TextField agnametextfield5i;
    @FXML
    private TextField agname2textfield5i;
    @FXML
    private TextField agbnummertextfield5i;
    @FXML
    private TextField agstnummertextfield5i;
    @FXML
    private TextField agstraßetextfield5i;
    @FXML
    private TextField aghausnummertextfield5i;
    @FXML
    private TextField agpostleitzahltextfield5i;
    @FXML
    private TextField agorttextfield5i;

    private String agname5i;
    private String agname25i;
    private String agbnummer5i;
    private String agstnummer5i;
    private String agstraße5i;
    private String aghausnummer5i;
    private String agpostleitzahl5i;
    private String agorttext5i;
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

    //Klick auf "Speichern"
    public void handlespeichernbutton5i(ActionEvent event) {
        //Werte speichern
        try {
            agname5i = agnametextfield5i.getText();
            agname25i = agname2textfield5i.getText();
            agbnummer5i = agbnummertextfield5i.getText();
            agstnummer5i = agstnummertextfield5i.getText();
            agstraße5i = agstraßetextfield5i.getText();
            aghausnummer5i = aghausnummertextfield5i.getText();
            agpostleitzahl5i = agpostleitzahltextfield5i.getText();
            agorttext5i = agorttextfield5i.getText();

            //Daten abspeichern
            SQLConnectionBase connection = new SQLConnectionBase();
            String id = ANW99PopupController.anid;
            connection.updateConstContent("arbeitnehmerkonstanten", id, anvorname3i, annachnamename3i, angeburtsnamename3i, anstraße3i, anhausnummer3i, anpostleitzahl3i, anort3i, angeburtsdatum3i, angeschlecht, anstaatssngehörigkeit3i, anpersonalnummer3i, ansvnummer3i, antätigkeit, anbgr, anberufsbezeichnung3i, anpgruppet, anstid, angleit, anbeschäftigungsbeginn3i, anmtlv, ankv, anrv, anu1, anu2, aninso, anst, agname5i, agbnummer5i, agstnummer5i, agstraße5i, aghausnummer5i, agpostleitzahl5i, agorttext5i, agname25i, anrhv);

            Main.changeScene("/Home1.fxml");
            CONFIRMPopup confirmPopup = new CONFIRMPopup();
            confirmPopup.display("CONFIRM99.fxml");

        }
        catch (Exception e) {
            e.printStackTrace();
            try{
                Main.changeScene("/Home1.fxml");
                CONFIRMPopup confirmPopup = new CONFIRMPopup();
                confirmPopup.display("NOTCONFIRM99.fxml");
            }
            catch (Exception e1) {
                e1.printStackTrace();
            }
        }
    }

    //Klick auf "-"
    public void handleklbutton5i(ActionEvent event) {
        try {
            Main.minimizeWindow();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "x"
    public void handleclbutton5i(ActionEvent event) {
        try {
            Main.openClosePopup("/ClosePopup99.fxml");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "Zurück"
    public void handlebackbutton5i(ActionEvent event) {
        try {
            Main.changeScene("/ANBE4.fxml");
        }
        catch (Exception e) {
            e.printStackTrace();
        }
    }


    //TextFiels setzen
    public void setagname5i(String text) {
        if (agnametextfield5i != null)
            agnametextfield5i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setagvertretung5i(String text) {
        if (agname2textfield5i != null)
            agname2textfield5i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setagbetriebsnr5i(String text) {
        if (agbnummertextfield5i != null)
        agbnummertextfield5i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setagstnr5i(String text) {
        if (agstnummertextfield5i != null)
            agstnummertextfield5i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setagstraße5i(String text) {
        if (agstraßetextfield5i != null)
            agstraßetextfield5i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setaghausnummer5i(String text) {
        if (aghausnummertextfield5i != null)
            aghausnummertextfield5i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setagpostleitzahl5i(String text) {
        if (agpostleitzahltextfield5i != null)
            agpostleitzahltextfield5i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }
    public void setagort5i(String text) {
        if (agorttextfield5i != null)
            agorttextfield5i.setText(text);
        else {
            System.out.println("Label is not initialized.");
        }
    }

    //Variablen holen
    public void setAnvorname3i(String anvorname3i) {
        this.anvorname3i = anvorname3i;
    }

    public void setAnnachnamename3i(String annachnamename3i) {
        this.annachnamename3i = annachnamename3i;
    }

    public void setAngeburtsnamename3i(String angeburtsnamename3i) {
        this.angeburtsnamename3i = angeburtsnamename3i;
    }

    public void setAnstraße3i(String anstraße3i) {
        this.anstraße3i = anstraße3i;
    }

    public void setAnhausnummer3i(String anhausnummer3i) {
        this.anhausnummer3i = anhausnummer3i;
    }

    public void setAnpostleitzahl3i(String anpostleitzahl3i) {
        this.anpostleitzahl3i = anpostleitzahl3i;
    }

    public void setAnort3i(String anort3i) {
        this.anort3i = anort3i;
    }

    public void setAngeburtsdatum3i(String angeburtsdatum3i) {
        this.angeburtsdatum3i = angeburtsdatum3i;
    }

    public void setAnstaatssngehörigkeit3i(String anstaatssngehörigkeit3i) {
        this.anstaatssngehörigkeit3i = anstaatssngehörigkeit3i;
    }

    public void setAnpersonalnummer3i(String anpersonalnummer3i) {
        this.anpersonalnummer3i = anpersonalnummer3i;
    }

    public void setAnsvnummer3i(String ansvnummer3i) {
        this.ansvnummer3i = ansvnummer3i;
    }

    public void setAnberufsbezeichnung3i(String anberufsbezeichnung3i) {
        this.anberufsbezeichnung3i = anberufsbezeichnung3i;
    }

    public void setAnbeschäftigungsbeginn3i(String anbeschäftigungsbeginn3i) {
        this.anbeschäftigungsbeginn3i = anbeschäftigungsbeginn3i;
    }

    public void setAngeschlecht(String angeschlecht) {
        this.angeschlecht = angeschlecht;
    }

    public void setAntätigkeit(String antätigkeit) {
        this.antätigkeit = antätigkeit;
    }

    public void setAnrhv(String anrhv) {
        this.anrhv = anrhv;
    }

    public void setAnbgr(String anbgr) {
        this.anbgr = anbgr;
    }

    public void setAnpgruppet(String anpgruppet) {
        this.anpgruppet = anpgruppet;
    }

    public void setAngleit(String angleit) {
        this.angleit = angleit;
    }

    public void setAnstid(String anstid) {
        this.anstid = anstid;
    }

    public void setAnmtlv(String anmtlv) {
        this.anmtlv = anmtlv;
    }

    public void setAnkv(String ankv) {
        this.ankv = ankv;
    }

    public void setAnrv(String anrv) {
        try {
            this.anrv = anrv;
            System.out.println("Geht");
        }
        catch (Exception e) {
            System.out.println("Geht nicht");
        }

    }

    public void setAnu1(String anu1) {
        this.anu1 = anu1;
    }

    public void setAnu2(String anu2) {
        this.anu2 = anu2;
    }

    public void setAninso(String aninso) {
        this.aninso = aninso;
    }

    public void setAnst(String anst) {
        this.anst = anst;
    }
}
