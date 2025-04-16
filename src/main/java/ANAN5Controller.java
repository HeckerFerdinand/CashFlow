import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.scene.control.TextField;
import java.io.File;

public class ANAN5Controller {


    @FXML
    private TextField agnametextfield5h;
    @FXML
    private TextField agname2textfield5h;
    @FXML
    private TextField agbnummertextfield5h;
    @FXML
    private TextField agstnummertextfield5h;
    @FXML
    private TextField agstraßetextfield5h;
    @FXML
    private TextField aghausnummertextfield5h;
    @FXML
    private TextField agpostleitzahltextfield5h;
    @FXML
    private TextField agorttextfield5h;

    private String agname5h;
    private String agname25h;
    private String agbnummer5h;
    private String agstnummer5h;
    private String agstraße5h;
    private String aghausnummer5h;
    private String agpostleitzahl5h;
    private String agorttext5h;
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

    //Klick auf "Speichern"
    public void handlespeichernbutton5h(ActionEvent event) {
        //Werte speichern
        try {
            agname5h = agnametextfield5h.getText();
            agname25h = agname2textfield5h.getText();
            agbnummer5h = agbnummertextfield5h.getText();
            agstnummer5h = agstnummertextfield5h.getText();
            agstraße5h = agstraßetextfield5h.getText();
            aghausnummer5h = aghausnummertextfield5h.getText();
            agpostleitzahl5h = agpostleitzahltextfield5h.getText();
            agorttext5h = agorttextfield5h.getText();

            //Daten abspeichern
            SQLConnectionBase connection = new SQLConnectionBase();
            String stringcount = connection.selectmaxConstContent("ids", "allids");
            String count;
            if (stringcount == null) {
                count = "01";
            }
            else {
                int intcount = Integer.parseInt(stringcount) + 1;
                count = String.valueOf(intcount);
                if (Double.parseDouble(count) < 10) {
                    count = "0".concat(count);
                }
            }

            connection.insertintoConstCalc("arbeitnehmerkonstanten", count, anvorname3h, annachnamename3h, angeburtsnamename3h, anstraße3h, anhausnummer3h, anpostleitzahl3h, anort3h, angeburtsdatum3h, angeschlecht, anstaatssngehörigkeit3h, anpersonalnummer3h, ansvnummer3h, antätigkeit, anbgr, anberufsbezeichnung3h, anpgruppet, anstid, angleit, anbeschäftigungsbeginn3h, anmtlv, ankv, anrv, anu1, anu2, aninso, anst, agname5h, agbnummer5h, agstnummer5h, agstraße5h, aghausnummer5h, agpostleitzahl5h, agorttext5h, agname25h, anrhv);
            connection.insertintoAllIdsTable(count);
            //mtl. Datenbank erstellen
            connection.createmtlTable(count);

            //Ordner in Dateien erstellen
            String directoryPath = "C:/Users/Anwender/IdeaProjects/000HVHecker/CashFlow/" + connection.selectConstContent("arbeitnehmerkonstanten", "anpersonalnummer", count) + " " + connection.selectConstContent("arbeitnehmerkonstanten", "annachname", count);
            File directory = new File(directoryPath);
            directory.mkdirs();

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
    public void handleklbutton5h(ActionEvent event) {
        try {
            Main.minimizeWindow();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "x"
    public void handleclbutton5h(ActionEvent event) {
        try {
            Main.openClosePopup("/ClosePopup99.fxml");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "Zurück"
    public void handlebackbutton5h(ActionEvent event) {
        try {
            Main.changeScene("/ANAN4.fxml");
        }
        catch (Exception e) {
            e.printStackTrace();
        }
    }

    public void setAnrhv(String anrhv) {
        this.anrhv = anrhv;
    }

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

    public void setAntätigkeit(String antätigkeit) {
        this.antätigkeit = antätigkeit;
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
        this.anrv = anrv;
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
