import javafx.event.ActionEvent;


public class Home1Controller {


    //Klick auf "Lohnbuchhaltung"
    public void handlelbbutton1() {
        try {
            Main.changeScene("/LAZA2.fxml");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "Stammdaten"
    public void handlesdvbutton1() {
        try {
            Main.changeScene("/ABLE2.fxml");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "Update-Protokoll"
    public void handlenbbutton1() {
        try {
            Main.openProtokoll();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "Impressum"
    public void handleipbutton1() {
        try {
            Main.openImpressum();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "-"
    public void handleklbutton1() {
        try {
            Main.minimizeWindow();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "x"
    public void handleclbutton1(ActionEvent event) {
        try {
            Main.openClosePopup("/ClosePopup99.fxml");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}




