import javafx.stage.Stage;


public class ClosePopup99Controller {


    private Stage popupwindow;


    //Klick auf "Schließen"
    public void handlescbuttonz()
    {
            Main.closeWindow();
            popupwindow.close();
            System.exit(0);
    }

    //Klick auf "Zurück"
    public void handlezubuttonz()
    {
        if(popupwindow != null)
        {
            popupwindow.close();
        }
    }

    public void setPopupwindow(Stage popupwindow) {
        this.popupwindow = popupwindow;
    }
}
