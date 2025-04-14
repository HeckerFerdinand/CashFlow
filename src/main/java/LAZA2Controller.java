import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import java.io.IOException;
import java.time.LocalDate;
import java.time.format.DateTimeFormatter;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;


public class LAZA2Controller {


    //Klick auf "Lohnabrechnung"
    @FXML
    public void handlelerfbutton2() {
        Main.openANWPopup("/ANW99.fxml", () -> {
            try {
                Main.changeScene("/LERF3.fxml");
                String anid = ANW99PopupController.anid;
                SQLConnectionBase sqlConnection = new SQLConnectionBase();
                String annamelabel3a = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anvorname", anid) + " " + sqlConnection.selectConstContent("arbeitnehmerkonstanten","annachname", anid);
                String pnrlabel3a = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anpersonalnummer", anid);
                String agnamelabel3a = sqlConnection.selectConstContent("arbeitnehmerkonstanten","agname", anid);
                String bnrlabel3a = sqlConnection.selectConstContent("arbeitnehmerkonstanten","agbetriebsnummer", anid);
                String mvlabel3a = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anmtlverguetung", anid) + "€";
                String rhlabel3a = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anregiestundenverguetung", anid) + "€/ Std.";
                LocalDate localDate = LocalDate.now();
                String daylabel3a = localDate.format(DateTimeFormatter.ofPattern("dd.MM.yyyy"));
                Main.setLERF3Content(annamelabel3a, pnrlabel3a, agnamelabel3a, bnrlabel3a, mvlabel3a, daylabel3a, rhlabel3a);
            }   catch (IOException e) {
                e.printStackTrace(); }
        });
    }

    //Klick auf "Zeitabrechnung"
    @FXML
    public void handlezerfbutton2() {
        Main.openANWPopup("/ANW99.fxml", () -> {
            try {
                Main.changeScene("/ZERF3.fxml");
                String anid = ANW99PopupController.anid;
                SQLConnectionBase sqlConnection = new SQLConnectionBase();
                String annamelabel3e = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anvorname", anid) + " " + sqlConnection.selectConstContent("arbeitnehmerkonstanten","annachname", anid);
                String pnrlabel3e = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anpersonalnummer", anid);
                String agnamelabel3e = sqlConnection.selectConstContent("arbeitnehmerkonstanten","agname", anid);
                String bnrlabel3e = sqlConnection.selectConstContent("arbeitnehmerkonstanten","agbetriebsnummer", anid);
                String hlabel3a = sqlConnection.selectConstContent("arbeitnehmerkonstanten","anmtlverguetung", anid) + "€";
                LocalDate localDate = LocalDate.now();
                String daylabel3a = localDate.format(DateTimeFormatter.ofPattern("dd.MM.yyyy"));
                Main.setZERF3Content(annamelabel3e, pnrlabel3e, agnamelabel3e, bnrlabel3e, hlabel3a, daylabel3a);
            }   catch (IOException e) {
                e.printStackTrace(); }
        });
    }

    //Klick auf "Lohnjournal jährl."
    @FXML
    public void handleljbutton2() {
        Main.openANWPopup("/ANW99.fxml", () -> {
            try {
                String anid = ANW99PopupController.anid;
                PDFjährlohnjournal pdFjährlohnjournal = new PDFjährlohnjournal();
                LocalDate localDate = LocalDate.now();
                String datum = localDate.format(DateTimeFormatter.ofPattern("dd.MM.yyyy"));
                String sjahr = localDate.format(DateTimeFormatter.ofPattern("yyyy"));
                int djahr = Integer.parseInt(sjahr) - 1;
                String jahr = String.valueOf(djahr);
                pdFjährlohnjournal.print(anid, datum, jahr);
            }
            catch (Exception e) {
                e.printStackTrace();
            }
        });
    }

    //Klick auf "Fortschritt einsehen"
    @FXML
    public void fortschrittbutton2(ActionEvent event) {
        try {
            SQLConnectionBase sqlConnection = new SQLConnectionBase();
            List<String> ids = sqlConnection.selectAllIds("arbeitnehmerkonstanten");
            ArrayList<String> anlist = new ArrayList<>();
            ArrayList<ArrayList> lohnlist = new ArrayList<ArrayList>();
            ArrayList<ArrayList> zeitlist = new ArrayList<ArrayList>();
            Map<String, ArrayList<Boolean>> lohnMap = new HashMap<String, ArrayList<Boolean>>();
            Map<String, ArrayList<Boolean>> zeitMap = new HashMap<String, ArrayList<Boolean>>();
            for (int i = 0; i < ids.size(); i++) {
                String zeitkey = "zeitkey" + ids.get(i);
                ArrayList<Boolean> zeitboolean = new ArrayList<Boolean>();
                for (int j = 0; j < 13; j++) {
                    String month = "";
                    LocalDate localDate = LocalDate.now();
                    String year = localDate.format(DateTimeFormatter.ofPattern("yyyy"));
                    switch (j) {
                        case 0:
                            month = "Januar";
                            break;
                        case 1:
                            month = "Februar";
                            break;
                        case 2:
                            month = "März";
                            break;
                        case 3:
                            month = "April";
                            break;
                        case 4:
                            month = "Mai";
                            break;
                        case 5:
                            month = "Juni";
                            break;
                        case 6:
                            month = "Juli";
                            break;
                        case 7:
                            month = "August";
                            break;
                        case 8:
                            month = "September";
                            break;
                        case 9:
                            month = "Oktober";
                            break;
                        case 10:
                            month = "November";
                            break;
                        case 11:
                            month = "Dezember";
                            break;
                        case 12:
                            month = "Januar";
                            break;
                            default:
                                month = "";
                    }
                    boolean zeitexists = sqlConnection.selectzeitexists(ids.get(i), month, year);
                    zeitboolean.add(zeitexists);
                }
                zeitMap.put(zeitkey, zeitboolean);


                String lohnkey = "lohnkey" + ids.get(i);
                ArrayList<Boolean> lohnboolean = new ArrayList<Boolean>();
                for (int j = 0; j < 13; j++) {
                    String month = "";
                    LocalDate localDate = LocalDate.now();
                    String year = localDate.format(DateTimeFormatter.ofPattern("yyyy"));
                    switch (j) {
                        case 0:
                            month = "Januar";
                            break;
                        case 1:
                            month = "Februar";
                            break;
                        case 2:
                            month = "März";
                            break;
                        case 3:
                            month = "April";
                            break;
                        case 4:
                            month = "Mai";
                            break;
                        case 5:
                            month = "Juni";
                            break;
                        case 6:
                            month = "Juli";
                            break;
                        case 7:
                            month = "August";
                            break;
                        case 8:
                            month = "September";
                            break;
                        case 9:
                            month = "Oktober";
                            break;
                        case 10:
                            month = "November";
                            break;
                        case 11:
                            month = "Dezember";
                            break;
                        case 12:
                            month = "Januar";
                            break;
                        default:
                            month = "";
                    }
                    boolean lohnexists = sqlConnection.selectlohnexists(ids.get(i), month, year);
                    lohnboolean.add(lohnexists);
                }
                lohnMap.put(lohnkey, lohnboolean);

                anlist.add(sqlConnection.selectConstContent("arbeitnehmerkonstanten", "anpersonalnummer", ids.get(i)).concat(" ").concat(sqlConnection.selectConstContent("arbeitnehmerkonstanten", "anvorname", ids.get(i))).concat(" ").concat(sqlConnection.selectConstContent("arbeitnehmerkonstanten", "annachname", ids.get(i))));
                lohnlist.add(lohnMap.get(lohnkey));
                zeitlist.add(zeitMap.get(zeitkey));
            }
            Main.setFORT3Content(anlist, lohnlist, zeitlist);
        }
        catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "-"
    @FXML
    public void handleklbutton2a(ActionEvent event) {
        try {
            Main.minimizeWindow();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "x"
    @FXML
    public void handleclbutton2a(ActionEvent event) {
        try {
            Main.openClosePopup("/ClosePopup99.fxml");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "Zurück"
    @FXML
    public void handlebackbutton2a() {
        try {
            Main.changeScene("/Home1.fxml");
        }
        catch (Exception e) {
            e.printStackTrace();
        }
    }

}




