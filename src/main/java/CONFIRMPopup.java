import javafx.fxml.FXMLLoader;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.stage.Stage;
import javafx.stage.StageStyle;
import javafx.util.Duration;
import javafx.animation.PauseTransition;

public class CONFIRMPopup {


    private Stage window4 = new Stage();

    public Stage display(String FXMLResource) {
            try {

                window4.setTitle("");
                Parent scene1 = FXMLLoader.load(getClass().getResource(FXMLResource));
                window4.setScene(new Scene(scene1));
                window4.initStyle(StageStyle.UNDECORATED);
                window4.setX(1580);
                window4.setY(300);
                window4.show();

                PauseTransition delay = new PauseTransition(Duration.seconds(5));
                delay.setOnFinished(event -> window4.close());
                delay.play();
            }
            catch (Exception e) {
                e.printStackTrace();
            }
            return window4;
        }
    }
