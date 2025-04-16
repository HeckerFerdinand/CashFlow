import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.scene.control.Button;
import javafx.scene.control.ChoiceBox;
import javafx.scene.control.Label;
import javafx.scene.control.TextField;
import java.util.ArrayList;

public class LODR3Controller {

    @FXML
    private javafx.scene.control.Label annamelabel3b;
    @FXML
    private javafx.scene.control.Label pnrlabel3b;
    @FXML
    private javafx.scene.control.Label agnamelabel3b;
    @FXML
    private javafx.scene.control.Label bnrlabel3b;
    @FXML
    private javafx.scene.control.Label mvlabel3b;
    @FXML
    private Label rhlabel3b;
    @FXML
    private TextField daytextfield3b;
    @FXML
    private ChoiceBox<String> yearchoicebox3b;
    @FXML
    private ChoiceBox<String> monthchoicebox3b;
    @FXML
    private Button sabutton3b;

    private String year3b;
    private String month3b;
    private String date;
    private String tableid;

    private ArrayList<String> yearlist;
    private ArrayList<String> monthlist;


    public void handlesabutton3b(ActionEvent event) {
        try{
            date = daytextfield3b.getText();
            year3b = yearchoicebox3b.getValue();
            month3b = monthchoicebox3b.getValue();
            tableid = ANW99PopupController.anid;
            PDFmtllohnabrechnung lohnabrechnung = new PDFmtllohnabrechnung();
            lohnabrechnung.print(tableid, date, month3b, year3b);
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
    public void handleklbutton3b(ActionEvent event) {
        try {
            Main.minimizeWindow();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "x"
    public void handleclbutton3b(ActionEvent event) {
        try {
            Main.openClosePopup("/ClosePopup99.fxml");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "Zurück"
    public void handlebackbutton3b(ActionEvent event) {
        try {
            Main.changeScene("/LAZA2.fxml");
        }
        catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Labels setzen
    public void setannamelabel3b(String text) {
        if (annamelabel3b != null) {
            annamelabel3b.setText(text);
        }
        else {
            System.out.println("nameLabel is not initialized.");
        }
    }
    public void setpnrlabel3b(String text) {
        if (pnrlabel3b != null) {
            pnrlabel3b.setText(text);
        }
        else {
            System.out.println("pnrLabel is not initialized.");
        }
    }
    public void setagnamelabel3b(String text) {
        if (agnamelabel3b != null) {
            agnamelabel3b.setText(text);
        }
        else {
            System.out.println("agnameLabel is not initialized.");
        }
    }
    public void setbnrlabel3b(String text) {
        if (bnrlabel3b != null) {
            bnrlabel3b.setText(text);
        }
        else {
            System.out.println("bnrLabel is not initialized.");
        }
    }
    public void setmvlabel3b(String text) {
        if (mvlabel3b != null) {
            mvlabel3b.setText(text);
        }
        else {
            System.out.println("mvLabel is not initialized.");
        }
    }
    public void setdaylabel3b(String text) {
        if (daytextfield3b != null) {
            daytextfield3b.setText(text);
        }
        else {
            System.out.println("dayLabel is not initialized.");
        }
    }
    public void setrhlabel3b(String text) {
        if (rhlabel3b != null) {
            rhlabel3b.setText(text);
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
        yearchoicebox3b.getItems().addAll(yearlist);
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
        monthchoicebox3b.getItems().addAll(monthlist);
    }

    public void setupBasicStyling1() {
        // CSS-Klassen direkt zuweisen
        yearchoicebox3b.getStyleClass().add("monthyear-choicebox");

        // Stylesheet erzwingen
        yearchoicebox3b.sceneProperty().addListener((obs, oldScene, newScene) -> {
            if (newScene != null) {
                newScene.getStylesheets().add(getClass().getResource("/Home1.css").toExternalForm());
            }
        });
    }

    public void setupBasicStyling2() {
        // CSS-Klassen direkt zuweisen
        monthchoicebox3b.getStyleClass().add("monthyear-choicebox");

        // Stylesheet erzwingen
        monthchoicebox3b.sceneProperty().addListener((obs, oldScene, newScene) -> {
            if (newScene != null) {
                newScene.getStylesheets().add(getClass().getResource("/Home1.css").toExternalForm());
            }
        });
    }
}

