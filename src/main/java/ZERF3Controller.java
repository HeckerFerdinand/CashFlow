import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.scene.control.Button;
import javafx.scene.control.ChoiceBox;
import javafx.scene.control.Label;
import javafx.scene.control.TextField;
import java.util.ArrayList;


public class ZERF3Controller {


    @FXML
    private Label annamelabel3e;
    @FXML
    private Label pnrlabel3e;
    @FXML
    private Label agnamelabel3e;
    @FXML
    private Label bnrlabel3e;
    @FXML
    private Label mvlabel3e;
    @FXML
    private TextField daytextfield3e;
    @FXML
    private TextField aztextfield3e;
    @FXML
    private TextField azrtextfield3e;
    @FXML
    private TextField azuetextfield3e;
    @FXML
    private TextField uttextfield3e;
    @FXML
    private TextField kttextfield3e;
    @FXML
    private ChoiceBox<String> yearchoicebox3e;
    @FXML
    private ChoiceBox<String> monthchoicebox3e;
    @FXML
    private Button sabutton3e;

    private String year3e;
    private String month3e;
    private String date;
    private String year;
    private String arbeitszeit3e;
    private String arbeitszeitregie3e;
    private String arbeitszeituebertrag3e;
    private String urlaubstage3e;
    private String krankheitstage;
    private String tableid;

    private ArrayList<String> yearlist;
    private ArrayList<String> monthlist;

    boolean u1;
    boolean u2;
    boolean u3;
    boolean u4;
    boolean u5;
    boolean u6;


    //Klick auf "Speichern"
    public void handlesabutton3e(ActionEvent event) {
        try {
            //Daten aus GUI extrahieren
            date = daytextfield3e.getText();
            year3e = yearchoicebox3e.getValue();
            month3e = monthchoicebox3e.getValue();
            arbeitszeit3e = aztextfield3e.getText();
            arbeitszeitregie3e = azrtextfield3e.getText();
            arbeitszeituebertrag3e = "0";
            urlaubstage3e = uttextfield3e.getText();
            krankheitstage = kttextfield3e.getText();
            tableid = ANW99PopupController.anid;
            //zusätzliche Daten berechnen
            Rechenzentrum rechenzentrum = new Rechenzentrum();
            String gesamtarbeitszeit = rechenzentrum.calcgesamtarbeitszeitzeit(arbeitszeit3e, arbeitszeitregie3e);
            //Daten in Database speichern
            SQLConnectionBase connection = new SQLConnectionBase();
            u1 = connection.updateContent(ANW99PopupController.anid, "arbeitszeit", month3e, year3e, arbeitszeit3e);
            u2 = connection.updateContent(ANW99PopupController.anid, "arbeitszeitregie", month3e, year3e, arbeitszeitregie3e);
            u3 = connection.updateContent(ANW99PopupController.anid, "arbeitszeituebertrag", month3e, year3e, arbeitszeituebertrag3e);
            u4 = connection.updateContent(ANW99PopupController.anid, "gesamtarbeitszeit", month3e, year3e, gesamtarbeitszeit);
            u5 = connection.updateContent(ANW99PopupController.anid, "urlaub", month3e, year3e, urlaubstage3e);
            u6 = connection.updateContent(ANW99PopupController.anid, "krank", month3e, year3e, krankheitstage);


        }
        catch (Exception e) {
            e.printStackTrace();
        }
        finally{
            if (u1 && u2 && u3 && u4 && u5 && u6) {
                try{
                    Main.changeScene("/LAZA2.fxml");
                    CONFIRMPopup confirmPopup = new CONFIRMPopup();
                    confirmPopup.display("CONFIRM99.fxml");
                }
                catch (Exception e1) {
                    e1.printStackTrace();
                }
            }
            else {
                try{
                    Main.changeScene("/LAZA2.fxml");
                    CONFIRMPopup confirmPopup = new CONFIRMPopup();
                    confirmPopup.display("NOTCONFIRM99.fxml");
                }
                catch (Exception e2) {
                    e2.printStackTrace();
                }
            }
        }
    }

    //Klick auf "x"
    public void handleclbutton3e(ActionEvent event) {
        try {
            Main.openClosePopup("/ClosePopup99.fxml");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "-"
    public void handleklbutton3e(ActionEvent event) {
        try {
            Main.minimizeWindow();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "Zurück"
    public void handlebackbutton3e(ActionEvent event) {
        try {
            Main.changeScene("/LAZA2.fxml");
        }
        catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Labels setzen
    public void setannamelabel3e(String text) {
        if (annamelabel3e != null) {
            annamelabel3e.setText(text);
        }
        else {
            System.out.println("nameLabel is not initialized.");
        }
    }

    public void setpnrlabel3e(String text) {
        if (pnrlabel3e != null) {
            pnrlabel3e.setText(text);
        }
        else {
            System.out.println("pnrLabel is not initialized.");
        }
    }

    public void setagnamelabel3e(String text) {
        if (agnamelabel3e != null) {
            agnamelabel3e.setText(text);
        }
        else {
            System.out.println("agnameLabel is not initialized.");
        }
    }

    public void setbnrlabel3e(String text) {
        if (bnrlabel3e != null) {
            bnrlabel3e.setText(text);
        }
        else {
            System.out.println("bnrLabel is not initialized.");
        }
    }

    public void sethlabel3e(String text) {
        if (mvlabel3e != null) {
            mvlabel3e.setText(text);
        }
        else {
            System.out.println("mvLabel is not initialized.");
        }
    }

    public void setdaylabel3e(String text) {
        if (daytextfield3e != null) {
            daytextfield3e.setText(text);
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
        yearchoicebox3e.getItems().addAll(yearlist);
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
        monthchoicebox3e.getItems().addAll(monthlist);
    }
    public void setupBasicStyling1() {
        // CSS-Klassen direkt zuweisen
        yearchoicebox3e.getStyleClass().add("monthyear-choicebox");

        // Stylesheet erzwingen
        yearchoicebox3e.sceneProperty().addListener((obs, oldScene, newScene) -> {
            if (newScene != null) {
                newScene.getStylesheets().add(getClass().getResource("/Home1.css").toExternalForm());
            }
        });
    }

    public void setupBasicStyling2() {
        // CSS-Klassen direkt zuweisen
        monthchoicebox3e.getStyleClass().add("monthyear-choicebox");

        // Stylesheet erzwingen
        monthchoicebox3e.sceneProperty().addListener((obs, oldScene, newScene) -> {
            if (newScene != null) {
                newScene.getStylesheets().add(getClass().getResource("/Home1.css").toExternalForm());
            }
        });
    }
}
