import javafx.fxml.FXML;

public class IMPR99Controller {

    @FXML
    public void handleimprbutton99(){
        try {
            Main.closeImpressum();
        }
        catch (Exception e) {
            e.printStackTrace();
        }
    }
}
