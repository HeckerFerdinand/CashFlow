import javafx.event.ActionEvent;
import javafx.stage.Stage;


public class ANLB99Controller {

    private Stage popupwindow;


    public void handleabbbutton99b(ActionEvent event) {
        try {
            popupwindow.close();
            Main.changeScene("/ABLE2.fxml");
        }
        catch (Exception e) {
            e.printStackTrace();
        }
    }

    public void handlelöschenbutton99b(ActionEvent event)  {
        try {
            SQLConnectionBase sqlConnectionBase = new SQLConnectionBase();
            String id = ANW99PopupController.anid;
            sqlConnectionBase.deleteConstContent(id);
            //sqlConnectionBase.dropmtlTable(id);
            popupwindow.close();
            Main.changeScene("/Home1.fxml");
            CONFIRMPopup confirmPopup = new CONFIRMPopup();
            confirmPopup.display("CONFIRM99.fxml");
        }
        catch (Exception e) {
            e.printStackTrace();
            try {
                Main.changeScene("/LAZA2.fxml");
                CONFIRMPopup confirmPopup = new CONFIRMPopup();
                confirmPopup.display("NOTCONFIRM99.fxml");
            }
            catch (Exception e1) {
                e1.printStackTrace();
            }
        }
    }

    public void handleclbutton99b(ActionEvent event) {
        try {
            Main.openClosePopup("/ClosePopup99.fxml");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public void setPopupwindow(Stage popupwindow) {
        this.popupwindow = popupwindow;
    }
}
