import javafx.fxml.FXMLLoader;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.stage.Modality;
import javafx.stage.Stage;
import javafx.stage.StageStyle;

public class ANLBPopup {

    private Stage window4 = new Stage();
    private CallBack callback;

    public ANLBPopup(CallBack callback) {
        this.callback = callback;
    }

    public Stage display(String FXMLResource) {
        try {
            window4.initModality(Modality.APPLICATION_MODAL);
            window4.setTitle("");
            FXMLLoader loader = new FXMLLoader(getClass().getResource(FXMLResource));
            Parent scene2 = loader.load();

            // Controller setzen und Stage übergeben
            ANLB99Controller controller = loader.getController();
            controller.setPopupwindow(window4);

            Scene scene = new Scene(scene2);
            window4.setScene(scene);
            window4.initStyle(StageStyle.UNDECORATED);
            window4.showAndWait();

            // Callback ausführen, nachdem das Fenster geschlossen wurde
            if (callback != null) {
                callback.execute();
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
        return window4;
    }



}
