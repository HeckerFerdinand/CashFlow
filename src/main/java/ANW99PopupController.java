import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.fxml.Initializable;
import javafx.scene.control.ChoiceBox;
import javafx.stage.Stage;
import java.net.URL;
import java.util.*;

public class ANW99PopupController implements Initializable {

    private Stage popupwindow;

    @FXML
    private ChoiceBox<String> ANchoicebox99;

    private List<String> arbeitnehmerliste;
    public static String anid;

    @Override
    public void initialize(URL url, ResourceBundle resourceBundle) {
        initializeArbeitnehmerListe();
        setupBasicStyling();
        ANchoicebox99.setOnAction(this::handleChoiceBoxSelection);
    }

    private void initializeArbeitnehmerListe() {
        arbeitnehmerliste = new ArrayList<>();
        SQLConnectionBase connection = new SQLConnectionBase();
        List<String> ids = connection.selectAllIds("arbeitnehmerkonstanten");

        for (String id : ids) {
            String entry = id + " "
                    + connection.selectConstContent("arbeitnehmerkonstanten", "anvorname", id) + " "
                    + connection.selectConstContent("arbeitnehmerkonstanten", "annachname", id) + ", "
                    + connection.selectConstContent("arbeitnehmerkonstanten", "agname", id);
            arbeitnehmerliste.add(entry);
        }

        arbeitnehmerliste.sort(String::compareTo);
        ANchoicebox99.getItems().addAll(arbeitnehmerliste);
    }

    private void setupBasicStyling() {
        // CSS-Klassen direkt zuweisen
        ANchoicebox99.getStyleClass().add("custom-choicebox");

        // Stylesheet erzwingen
        ANchoicebox99.sceneProperty().addListener((obs, oldScene, newScene) -> {
            if (newScene != null) {
                newScene.getStylesheets().add(getClass().getResource("/Home1.css").toExternalForm());
            }
        });
    }

    @FXML
    private void handleChoiceBoxSelection(ActionEvent event) {
        String selection = ANchoicebox99.getValue();
        if (selection != null && !selection.isEmpty()) {
            anid = selection.substring(0, 2);
            if (popupwindow != null) {
                popupwindow.close();
            }
        }
    }

    @FXML
    private void handleclbutton99(ActionEvent actionEvent) {
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

/*import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.fxml.Initializable;
import javafx.scene.control.ChoiceBox;
import javafx.stage.Stage;
import java.net.URL;
import java.util.ArrayList;
import java.util.List;
import java.util.ResourceBundle;


public class ANW99PopupController implements Initializable {


    private Stage popupwindow;
    @FXML
    private ChoiceBox<String> ANchoicebox99;
    private ArrayList<String> arbeitnehmerliste;
    String arbeitnehmer;
    int countoutput;
    String an;
    public static String anid;


    //Choicebox erstellen
    @Override public void initialize(URL url, ResourceBundle resourceBundle) {
        arbeitnehmerliste = new ArrayList<>();
        SQLConnectionBase connection = new SQLConnectionBase();
        {
            List<String> ids = connection.selectAllIds("arbeitnehmerkonstanten");
            for (String id : ids) {
                an = connection.selectConstContent("arbeitnehmerkonstanten", "id", id).concat(" ").concat(connection.selectConstContent("arbeitnehmerkonstanten", "anvorname", id).concat(" ") .concat(connection.selectConstContent("arbeitnehmerkonstanten", "annachname", id)).concat(", ") .concat(connection.selectConstContent("arbeitnehmerkonstanten", "agname", id)));
                arbeitnehmerliste.add(an);}
            arbeitnehmerliste.sort(String::compareTo);
        }
        ANchoicebox99.getItems().addAll(arbeitnehmerliste);
        ANchoicebox99.getStyleClass().add("choicebox-anw-popup");
        ANchoicebox99.sceneProperty().addListener((obs, oldScene, newScene) -> {
            if (newScene != null) {
                newScene.windowProperty().addListener((observableWindow, oldWindow, newWindow) -> {
                    if (newWindow != null) {
                        newWindow.addEventHandler(javafx.stage.WindowEvent.WINDOW_SHOWN, event -> {
                            newScene.getStylesheets().add(getClass().getResource("/Home1.css").toExternalForm());
                        });
                    }
                });
            }
        });
        ANchoicebox99.setOnAction(this::getValue);
    }

    //Value extrahieren
    public void getValue(ActionEvent event) {
        arbeitnehmer = ANchoicebox99.getValue();
        anid = arbeitnehmer.substring(0, 2);
        popupwindow.close();
    }

    //Klick auf "x"
    public void handleclbutton99(ActionEvent actionEvent) {
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

*/
