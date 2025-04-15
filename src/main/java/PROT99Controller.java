import javafx.fxml.FXML;

public class PROT99Controller {

    @FXML
    public void handleprotbutton99(){
        try {
            Main.closeProtokoll();
        }
        catch (Exception e) {
            e.printStackTrace();
        }
    }
}

