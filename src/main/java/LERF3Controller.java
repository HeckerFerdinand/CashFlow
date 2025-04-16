import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.scene.control.Button;
import javafx.scene.control.ChoiceBox;
import javafx.scene.control.Label;
import javafx.scene.control.TextField;
import java.util.ArrayList;


public class LERF3Controller {


    @FXML
    private Label annamelabel3a;
    @FXML
    private Label pnrlabel3a;
    @FXML
    private Label agnamelabel3a;
    @FXML
    private Label bnrlabel3a;
    @FXML
    private Label mvlabel3a;
    @FXML
    private Label rhlabel3a;
    @FXML
    private TextField daytextfield3a;
    @FXML
    private TextField bbtextfield3a;
    @FXML
    private TextField sbbtextfield3a;
    @FXML
    private TextField abtextfield3a;
    @FXML
    private ChoiceBox<String> yearchoicebox3a;
    @FXML
    private ChoiceBox<String> monthchoicebox3a;
    @FXML
    private Button sabutton3a;

    private String year3a;
    private String month3a;
    private String date;
    private double bruttobezuege3a;
    private double sonstbruttobezuege3a;
    private double auszahlungsbetrag3a;
    private String tableid;

    private ArrayList<String> yearlist;
    private ArrayList<String> monthlist;


    //Klick auf "Speichern und Ausgeben"
    public void handlesabutton3a(ActionEvent event) {
        try {
            //Daten aus GUI extrahieren
            date = daytextfield3a.getText();
            year3a = yearchoicebox3a.getValue();
            month3a = monthchoicebox3a.getValue();
            bruttobezuege3a = Double.parseDouble(bbtextfield3a.getText().replace(",", "."));
            sonstbruttobezuege3a = Double.parseDouble(sbbtextfield3a.getText().replace(",", "."));
            auszahlungsbetrag3a = Double.parseDouble(abtextfield3a.getText().replace(",", "."));
            tableid = ANW99PopupController.anid;
            //Konstanten importieren
            SQLConnectionBase connection = new SQLConnectionBase();
            double kv = Double.parseDouble(connection.selectConstContent("arbeitnehmerkonstanten", "ankv", ANW99PopupController.anid));
            double rv = Double.parseDouble(connection.selectConstContent("arbeitnehmerkonstanten", "anrv", ANW99PopupController.anid));
            double u1 = Double.parseDouble(connection.selectConstContent("arbeitnehmerkonstanten", "anu1", ANW99PopupController.anid));
            double u2 = Double.parseDouble(connection.selectConstContent("arbeitnehmerkonstanten", "anu2", ANW99PopupController.anid));
            double inso = Double.parseDouble(connection.selectConstContent("arbeitnehmerkonstanten", "aninso", ANW99PopupController.anid));
            double st = Double.parseDouble(connection.selectConstContent("arbeitnehmerkonstanten", "anst", ANW99PopupController.anid));
            //zusätzliche Daten Berechnen
            Rechenzentrum rechenzentrum = new Rechenzentrum();
            double gesamtbetragbrutto = rechenzentrum.calcgesamtbetragbrutto(bruttobezuege3a, sonstbruttobezuege3a);
            double kva = rechenzentrum.calckva(gesamtbetragbrutto, kv);
            double rva = rechenzentrum.calcrva(gesamtbetragbrutto, rv);
            double u1a = rechenzentrum.calcu1a(gesamtbetragbrutto, u1);
            double u2a = rechenzentrum.calcu2a(gesamtbetragbrutto, u2);
            double insoa = rechenzentrum.calcinsoa(gesamtbetragbrutto, inso);
            double sta = rechenzentrum.calcsta(gesamtbetragbrutto, st);
            double gesamtbeitrag = rechenzentrum.calcgesamtbeitrag(kva, rva, u1a, u2a, insoa, sta);
            //Daten in Database speichern
            connection.insertintomtlCalc(tableid, year3a, month3a, bruttobezuege3a, sonstbruttobezuege3a, auszahlungsbetrag3a, gesamtbetragbrutto, kv, rv, u1, u2, inso, st, kva, rva, u1a, u2a, insoa, sta, gesamtbeitrag, date);
            //PDF erstellen und speichern
            PDFmtllohnabrechnung lohnabrechnung = new PDFmtllohnabrechnung();
            lohnabrechnung.print(tableid, date, month3a, year3a);
            //Szene schließen
            Main.changeScene("/LAZA2.fxml");
            CONFIRMPopup confirmPopup = new CONFIRMPopup();
            confirmPopup.display("CONFIRM99.fxml");
        }
        catch(Exception e){
            e.printStackTrace();
            try {
                Main.changeScene("/LAZA2.fxml");
            }
            catch(Exception e1) {e1.printStackTrace();}
            finally {
                CONFIRMPopup confirmPopup = new CONFIRMPopup();
                confirmPopup.display("NOTCONFIRM99.fxml");
            }
        }
    }

    //Klick auf "-"
    public void handleklbutton3a(ActionEvent event) {
        try {
            Main.minimizeWindow();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "x"
    public void handleclbutton3a(ActionEvent event) {
        try {
            Main.openClosePopup("/ClosePopup99.fxml");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "Zurück"
    public void handlebackbutton3a(ActionEvent event) {
        try {
            Main.changeScene("/LAZA2.fxml");
        }
        catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Labels setzen
    public void setannamelabel3a(String text) {
        if (annamelabel3a != null) {
            annamelabel3a.setText(text);
        }
        else {
            System.out.println("nameLabel is not initialized.");
        }
    }
    public void setpnrlabel3a(String text) {
        if (pnrlabel3a != null) {
            pnrlabel3a.setText(text);
        }
        else {
            System.out.println("pnrLabel is not initialized.");
        }
    }
    public void setagnamelabel3a(String text) {
        if (agnamelabel3a != null) {
            agnamelabel3a.setText(text);
        }
        else {
            System.out.println("agnameLabel is not initialized.");
        }
    }
    public void setbnrlabel3a(String text) {
        if (bnrlabel3a != null) {
            bnrlabel3a.setText(text);
        }
        else {
            System.out.println("bnrLabel is not initialized.");
        }
    }
    public void setmvlabel3a(String text) {
        if (mvlabel3a != null) {
            mvlabel3a.setText(text);
        }
        else {
            System.out.println("mvLabel is not initialized.");
        }
    }
    public void setdaylabel3a(String text) {
        if (daytextfield3a != null) {
            daytextfield3a.setText(text);
        }
        else {
            System.out.println("dayLabel is not initialized.");
        }
    }
    public void setrhlabel3a(String text) {
        if (rhlabel3a != null) {
            rhlabel3a.setText(text);
        }
        else {
            System.out.println("dayLabel is not initialized.");
        }
    }
    public void setyears()
    {
        yearlist = new ArrayList<>();
        yearlist.add("2025");
        yearlist.add("2026");
        yearlist.add("2027");
        yearlist.add("2028");
        yearlist.add("2029");
        yearlist.add("2030");
        yearchoicebox3a.getItems().addAll(yearlist);
    }
    public void setmonths()
    {
        monthlist = new ArrayList<>();
        monthlist.add("Januar");
        monthlist.add("Februar");
        monthlist.add("März");
        monthlist.add("April");
        monthlist.add("Mai");
        monthlist.add("Juni");
        monthlist.add("Juli");
        monthlist.add("August");
        monthlist.add("September");
        monthlist.add("Oktober");
        monthlist.add("November");
        monthlist.add("Dezember");
        monthchoicebox3a.getItems().addAll(monthlist);
    }

    public void setupBasicStyling1() {
        // CSS-Klassen direkt zuweisen
        yearchoicebox3a.getStyleClass().add("monthyear-choicebox");

        // Stylesheet erzwingen
        yearchoicebox3a.sceneProperty().addListener((obs, oldScene, newScene) -> {
            if (newScene != null) {
                newScene.getStylesheets().add(getClass().getResource("/Home1.css").toExternalForm());
            }
        });
    }

    public void setupBasicStyling2() {
        // CSS-Klassen direkt zuweisen
        monthchoicebox3a.getStyleClass().add("monthyear-choicebox");

        // Stylesheet erzwingen
        monthchoicebox3a.sceneProperty().addListener((obs, oldScene, newScene) -> {
            if (newScene != null) {
                newScene.getStylesheets().add(getClass().getResource("/Home1.css").toExternalForm());
            }
        });
    }
}
