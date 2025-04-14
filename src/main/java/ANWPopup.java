import javafx.fxml.FXMLLoader;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.stage.Modality;
import javafx.stage.Stage;
import javafx.stage.StageStyle;


public class ANWPopup {


    private Stage window3 = new Stage();
    private CallBack callback;


    public ANWPopup(CallBack callback) {
        this.callback = callback;
    }

    public Stage display(String FXMLResource) {
        try {
            window3.initModality(Modality.APPLICATION_MODAL);
            window3.setTitle("");
            FXMLLoader loader = new FXMLLoader(getClass().getResource(FXMLResource));
            Parent scene2 = loader.load();

            // Controller setzen und Stage übergeben
            ANW99PopupController controller = loader.getController();
            controller.setPopupwindow(window3);

            Scene scene = new Scene(scene2);
            window3.setScene(scene);
            window3.initStyle(StageStyle.UNDECORATED);
            window3.showAndWait();

            // Callback ausführen, nachdem das Fenster geschlossen wurde
            if (callback != null) {
                callback.execute();
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
        return window3;
    }
}
