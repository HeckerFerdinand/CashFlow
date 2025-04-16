import javafx.application.Application;
import javafx.scene.Scene;
import javafx.stage.Stage;
import javafx.stage.Screen;
import javafx.scene.*;
import javafx.fxml.FXMLLoader;
import javafx.geometry.Rectangle2D;
import javafx.stage.StageStyle;
import java.io.IOException;
import java.time.LocalDate;
import java.time.format.DateTimeFormatter;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;


public class Main extends Application {


    static Stage window1;
    static Stage window2;
    static Stage window3;
    private ANW99PopupController anw99Controller;


    public static void main(String[] args) {
        try {
            launch(args);
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    @Override
    public void start(Stage primaryStage){
        try{
            window1 = primaryStage;
            window1.setTitle("CashFlow v0.1");
            Parent scene1 = FXMLLoader.load(getClass().getResource("/Home1.fxml"));
            Rectangle2D screenBounds = Screen.getPrimary().getVisualBounds();
            primaryStage.setX(screenBounds.getMinX());
            primaryStage.setY(screenBounds.getMinY());
            primaryStage.setWidth(screenBounds.getWidth());
            primaryStage.setHeight(screenBounds.getHeight());
            window1.setScene(new Scene(scene1));
            window1.initStyle(StageStyle.UNDECORATED);
            window1.show();
        }
        catch(Exception e){
            e.printStackTrace();
        }
    }

    public static void openImpressum(){
        try{
            window2 = new Stage();
            Parent scene1 = FXMLLoader.load(Main.class.getResource("/IMPR99.fxml"));
            window2.setScene(new Scene(scene1));
            window2.initStyle(StageStyle.UNDECORATED);
            window2.show();
        }
        catch(IOException e){
            e.printStackTrace();
        }
    }

    public static void closeImpressum(){
        window2.close();
    }

    public static void openProtokoll(){
        try{
            window3 = new Stage();
            Parent scene1 = FXMLLoader.load(Main.class.getResource("/PROT99.fxml"));
            window3.setScene(new Scene(scene1));
            window3.initStyle(StageStyle.UNDECORATED);
            window3.show();
        }
        catch(IOException e){
            e.printStackTrace();
        }
    }

    public static void closeProtokoll(){
        window3.close();
    }

    public static void changeScene(String fxml) throws IOException {
        Parent scene2 = FXMLLoader.load(Main.class.getResource(fxml));
        window1.getScene().setRoot(scene2);
    }

    public static void openwindow(String title, String fxml) {
        try{
            Stage stage = new Stage();
            stage.setTitle(title);
            Parent scene = FXMLLoader.load(Main.class.getResource(fxml));
            stage.setScene(new Scene(scene));
            stage.show();
        }
        catch (Exception e){
            e.printStackTrace();
        }
    }

    public static Stage openClosePopup(String FXMLResource) {
        ClosePopup closePopup = new ClosePopup();
        return closePopup.display(FXMLResource);//
    }

    public static Stage openANWPopup(String FXMLResource, CallBack callback) {
        ANWPopup anwPopup = new ANWPopup(callback);
        return anwPopup.display(FXMLResource);
    }

    public static Stage openANLBPopup(String FXMLResource, CallBack callback) {
        ANLBPopup anlbPopup = new ANLBPopup(callback);
        return anlbPopup.display(FXMLResource);
    }

    public static void closeWindow() {
        window1.close();
    }

    public static void minimizeWindow() {
        window1.setIconified(true);
    }

    public static void setLERF3Content(String annamelabel3a, String pnrlabel3a, String agnamelabel3a, String bnrlabel3a, String mvlabel3a, String daylabel3a, String rhlabel3a) {
        try {
            FXMLLoader loader = new FXMLLoader(Main.class.getResource("/LERF3.fxml"));
            Parent root = loader.load();
            LERF3Controller controller = loader.getController();
            controller.setannamelabel3a(annamelabel3a);
            controller.setpnrlabel3a(pnrlabel3a);
            controller.setagnamelabel3a(agnamelabel3a);
            controller.setbnrlabel3a(bnrlabel3a);
            controller.setmvlabel3a(mvlabel3a);
            controller.setdaylabel3a(daylabel3a);
            controller.setrhlabel3a(rhlabel3a);
            controller.setyears();
            controller.setmonths();
            controller.setupBasicStyling1();
            controller.setupBasicStyling2();
            window1.getScene().setRoot(root); }
        catch (IOException e) {
            e.printStackTrace();
        }
    }

    public static void setZERF3Content(String annamelabel3e, String pnrlabel3e, String agnamelabel3e, String bnrlabel3e, String hlabel3e, String daylabel3e) {
        try {
            FXMLLoader loader = new FXMLLoader(Main.class.getResource("/ZERF3.fxml"));
            Parent root = loader.load();
            ZERF3Controller controller = loader.getController();
            controller.setannamelabel3e(annamelabel3e);
            controller.setpnrlabel3e(pnrlabel3e);
            controller.setagnamelabel3e(agnamelabel3e);
            controller.setbnrlabel3e(bnrlabel3e);
            controller.sethlabel3e(hlabel3e);
            controller.setdaylabel3e(daylabel3e);
            controller.setyears();
            controller.setmonths();
            controller.setupBasicStyling1();
            controller.setupBasicStyling2();
            window1.getScene().setRoot(root); }
        catch (IOException e) {
            e.printStackTrace();
        }
    }

    public static void setANBE3Content(String anvorname3i, String annachname3i, String angeburtsname3i, String anstraße3i, String anhausnummer3i, String anpostleitzahl3i, String anort3i, String angeburtsdatum3i, String anstaatsangehörigkeit3i, String anpersonalnummer3i, String ansvnummer3i, String anberufsbezeichnung3i, String anbeschäftigungsbeginn3i, String angeschlecht){
        try {
            FXMLLoader loader = new FXMLLoader(Main.class.getResource("/ANBE3.fxml"));
            loader.setControllerFactory(param -> StaticSingleController.getANBE3Controller());
            Parent root = loader.load();
            ANBE3Controller controller = loader.getController();
            controller.setanvornametextfield3i(anvorname3i);
            controller.setannachnametextfield3i(annachname3i);
            controller.setangeburtsnametextfield3i(angeburtsname3i);
            controller.setanstraßetextfield3i(anstraße3i);
            controller.setanhausnummertextfield3i(anhausnummer3i);
            controller.setanpostleitzahltextfield3i(anpostleitzahl3i);
            controller.setanorttextfield3i(anort3i);
            controller.setangeburtsdatumtextfield3i(angeburtsdatum3i);
            controller.setanstaatsangehörigkeittextfield3i(anstaatsangehörigkeit3i);
            controller.setanpersonalnummertextfield3i(anpersonalnummer3i);
            controller.setansvnummertextfield3i(ansvnummer3i);
            controller.setanberufsbezeichnungtextfield3i(anberufsbezeichnung3i);
            controller.setanbeschäftigungsbeginntextfield3i(anbeschäftigungsbeginn3i);
            if (angeschlecht.equals("männlich")){
                controller.setanmännlichcheckbox3i();
            }
            else if (angeschlecht.equals("weiblich")){
                controller.setanweiblichcheckbox3i();
            }
            else if (angeschlecht.equals("divers")) {
                controller.setandiverscheckbox3i();
            }
            window1.getScene().setRoot(root); }
        catch (IOException e) {
            e.printStackTrace();
        }

    }

    public static void setANBE4Content(String antäschl4i, String anbgrschl4i, String anpersgruppe4i, String anstid4i, String angleit4i, String anmtlv4i, String anstds4i, String ankv4i, String anrv4i, String anu14i, String anu24i, String aninso4i, String anst4i){
        try {
            FXMLLoader loader = new FXMLLoader(Main.class.getResource("/ANBE4.fxml"));
            loader.setControllerFactory(param -> StaticSingleController.getANBE4Controller());
            Parent root = loader.load();
            ANBE4Controller controller = loader.getController();
            controller.setantätigkeittextfield4i(antäschl4i);
            controller.setanbgrtextfield4i(anbgrschl4i);
            controller.setanpgruppetextfield4i(anpersgruppe4i);
            controller.setanstid(anstid4i);
            controller.setangleit(angleit4i);
            controller.setanmtlv4i(anmtlv4i);
            controller.setanstds4i(anstds4i);
            controller.setankv4i(ankv4i);
            controller.setanrv4i(anrv4i);
            controller.setanu14i(anu14i);
            controller.setanu24i(anu24i);
            controller.setaninso4i(aninso4i);
            controller.setanst4i(anst4i);
            window1.getScene().setRoot(root); }
        catch (IOException e) {
            e.printStackTrace();
        }
    }

    public static void setANBE5Content(String agname, String agvertretung, String agbetriebsnr, String agstnr, String agstraße, String aghausnummer, String agposteitzahl, String agort){
        try {
            FXMLLoader loader = new FXMLLoader(Main.class.getResource("/ANBE5.fxml"));
            loader.setControllerFactory(param -> StaticSingleController.getANBE5Controller());
            Parent root = loader.load();
            ANBE5Controller controller = loader.getController();
            controller.setagname5i(agname);
            controller.setagvertretung5i(agvertretung);
            controller.setagbetriebsnr5i(agbetriebsnr);
            controller.setagstnr5i(agstnr);
            controller.setagstraße5i(agstraße);
            controller.setaghausnummer5i(aghausnummer);
            controller.setagpostleitzahl5i(agposteitzahl);
            controller.setagort5i(agort);
            window1.getScene().setRoot(root); }
        catch (IOException e) {
            e.printStackTrace();
        }
    }

    public static void setFORT3Content(ArrayList<String> anlist, ArrayList<ArrayList> lohnlist, ArrayList<ArrayList> zeitlist) {
        try {
            FXMLLoader loader = new FXMLLoader(Main.class.getResource("/FORT3.fxml"));
            Parent root = loader.load();
            FORT3Controller controller = loader.getController();
            for (int i = 0; i < anlist.size(); i++) {
                controller.addRow(anlist.get(i), lohnlist.get(i), zeitlist.get(i));
            }
            window1.getScene().setRoot(root); }
        catch (IOException e) {
            e.printStackTrace();
        }
    }

    public ANW99PopupController getANW99Controller() {
        return anw99Controller;
    }
}








































