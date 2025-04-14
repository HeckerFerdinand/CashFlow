import javafx.fxml.FXMLLoader;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.stage.Modality;
import javafx.stage.Stage;
import javafx.stage.StageStyle;


public class ClosePopup {


    Stage window2 = new Stage();


    //Fenster erstellen + Sceneninhalt importieren
    public  Stage display(String fxmlresource){
        try {
            window2.initModality(Modality.APPLICATION_MODAL);
            window2.setTitle("");
            FXMLLoader loader = new FXMLLoader(getClass().getResource(fxmlresource));
            Parent scene2 = loader.load();
            ClosePopup99Controller controller = loader.getController();
            controller.setPopupwindow(window2);
            Scene scene = new Scene(scene2);
            window2.setScene(scene);
            window2.initStyle(StageStyle.UNDECORATED);
            window2.showAndWait();
        }
        catch(Exception e){
            e.printStackTrace();
        }
        return window2;
    }
}




























