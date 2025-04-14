import javafx.beans.property.BooleanProperty;
import javafx.beans.property.SimpleBooleanProperty;
import javafx.beans.property.SimpleStringProperty;
import javafx.beans.property.StringProperty;
import javafx.collections.FXCollections;
import javafx.collections.ObservableList;
import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.geometry.Insets;
import javafx.geometry.Pos;
import javafx.scene.control.*;
import javafx.scene.layout.HBox;
import javafx.util.Callback;

import java.util.ArrayList;
import java.util.function.Function;

public class FORT3Controller {

    @FXML private TableView<RowData> tableView;
    @FXML private CheckBox lohnabrcheckbox2c;
    @FXML private CheckBox zeitabrcheckbox2c;

    private final ObservableList<RowData> data = FXCollections.observableArrayList();

    public void initialize() {
        tableView.getStylesheets().add(getClass().getResource("Home1.css").toExternalForm());
        fillTable();
        setcheckboxes();
    }

    public void fillTable() {
        tableView.getColumns().clear();

        // Arbeiternehmer-Spalte
        TableColumn<RowData, String> nameColumn = new TableColumn<>("Arbeitnehmer");
        nameColumn.setCellValueFactory(cellData -> cellData.getValue().nameProperty());
        nameColumn.setPrefWidth(400);
        nameColumn.setStyle("-fx-font-size: 25px;" + "-fx-alignment: CENTER-LEFT;");

        // Monatsspalten
        String[] months = {"JAN", "FEB", "MÄR", "APR", "MAI", "JUN",
                "JUL", "AUG", "SEP", "OKT", "NOV", "DEZ", "JAN"};

        for (String month : months) {
            TableColumn<RowData, Void> monthColumn = new TableColumn<>(month);
            monthColumn.setPrefWidth(100);
            monthColumn.setCellFactory(getMonthCellFactory(month));
            tableView.getColumns().add(monthColumn);
        }
        tableView.setRowFactory(tv -> {
            TableRow<RowData> row = new TableRow<>();
            row.setStyle("-fx-border-color: black; -fx-border-width: 0 0 1 0;");
            return row;
        });

        tableView.getColumns().add(0, nameColumn);
        tableView.setStyle(
                "-fx-table-cell-border-color: black;" +
                        " -fx-border-color: black;" +
                        " -fx-border-width: 0.5;"
        );
        tableView.setItems(data);
    }

    private Callback<TableColumn<RowData, Void>, TableCell<RowData, Void>> getMonthCellFactory(String month) {
        return param -> new DoubleCheckBoxCell(
                row -> getCheck1Property(row, month),
                row -> getCheck2Property(row, month)
        );
    }

    private BooleanProperty getCheck1Property(RowData row, String month) {
        return switch (month) {
            case "JAN" -> row.janCheck1Property();
            case "FEB" -> row.febCheck1Property();
            case "MÄR" -> row.marCheck1Property();
            case "APR" -> row.aprCheck1Property();
            case "MAI" -> row.maiCheck1Property();
            case "JUN" -> row.junCheck1Property();
            case "JUL" -> row.julCheck1Property();
            case "AUG" -> row.augCheck1Property();
            case "SEP" -> row.sepCheck1Property();
            case "OKT" -> row.oktCheck1Property();
            case "NOV" -> row.novCheck1Property();
            case "DEZ" -> row.dezCheck1Property();
            default -> row.jan2Check1Property();
        };
    }

    private BooleanProperty getCheck2Property(RowData row, String month) {
        return switch (month) {
            case "JAN" -> row.janCheck2Property();
            case "FEB" -> row.febCheck2Property();
            case "MÄR" -> row.marCheck2Property();
            case "APR" -> row.aprCheck2Property();
            case "MAI" -> row.maiCheck2Property();
            case "JUN" -> row.junCheck2Property();
            case "JUL" -> row.julCheck2Property();
            case "AUG" -> row.augCheck2Property();
            case "SEP" -> row.sepCheck2Property();
            case "OKT" -> row.oktCheck2Property();
            case "NOV" -> row.novCheck2Property();
            case "DEZ" -> row.dezCheck2Property();
            default -> row.jan2Check2Property();
        };
    }

    public void addRow(String name, ArrayList<Boolean> checks1, ArrayList<Boolean> checks2) {
        data.add(new RowData(
                name,
                checks1.get(0), checks2.get(0),
                checks1.get(1), checks2.get(1),
                checks1.get(2), checks2.get(2),
                checks1.get(3), checks2.get(3),
                checks1.get(4), checks2.get(4),
                checks1.get(5), checks2.get(5),
                checks1.get(6), checks2.get(6),
                checks1.get(7), checks2.get(7),
                checks1.get(8), checks2.get(8),
                checks1.get(9), checks2.get(9),
                checks1.get(10), checks2.get(10),
                checks1.get(11), checks2.get(11),
                checks1.get(12), checks2.get(12)
        ));
    }


    public static class RowData {
        private final StringProperty name;

        // Alle BooleanProperties
        private final BooleanProperty janCheck1 = new SimpleBooleanProperty();
        private final BooleanProperty janCheck2 = new SimpleBooleanProperty();
        private final BooleanProperty febCheck1 = new SimpleBooleanProperty();
        private final BooleanProperty febCheck2 = new SimpleBooleanProperty();
        private final BooleanProperty marCheck1 = new SimpleBooleanProperty();
        private final BooleanProperty marCheck2 = new SimpleBooleanProperty();
        private final BooleanProperty aprCheck1 = new SimpleBooleanProperty();
        private final BooleanProperty aprCheck2 = new SimpleBooleanProperty();
        private final BooleanProperty maiCheck1 = new SimpleBooleanProperty();
        private final BooleanProperty maiCheck2 = new SimpleBooleanProperty();
        private final BooleanProperty junCheck1 = new SimpleBooleanProperty();
        private final BooleanProperty junCheck2 = new SimpleBooleanProperty();
        private final BooleanProperty julCheck1 = new SimpleBooleanProperty();
        private final BooleanProperty julCheck2 = new SimpleBooleanProperty();
        private final BooleanProperty augCheck1 = new SimpleBooleanProperty();
        private final BooleanProperty augCheck2 = new SimpleBooleanProperty();
        private final BooleanProperty sepCheck1 = new SimpleBooleanProperty();
        private final BooleanProperty sepCheck2 = new SimpleBooleanProperty();
        private final BooleanProperty oktCheck1 = new SimpleBooleanProperty();
        private final BooleanProperty oktCheck2 = new SimpleBooleanProperty();
        private final BooleanProperty novCheck1 = new SimpleBooleanProperty();
        private final BooleanProperty novCheck2 = new SimpleBooleanProperty();
        private final BooleanProperty dezCheck1 = new SimpleBooleanProperty();
        private final BooleanProperty dezCheck2 = new SimpleBooleanProperty();
        private final BooleanProperty jan2Check1 = new SimpleBooleanProperty();
        private final BooleanProperty jan2Check2 = new SimpleBooleanProperty();

        public RowData(String name,
                       boolean jan1, boolean jan2,
                       boolean feb1, boolean feb2,
                       boolean mar1, boolean mar2,
                       boolean apr1, boolean apr2,
                       boolean mai1, boolean mai2,
                       boolean jun1, boolean jun2,
                       boolean jul1, boolean jul2,
                       boolean aug1, boolean aug2,
                       boolean sep1, boolean sep2,
                       boolean okt1, boolean okt2,
                       boolean nov1, boolean nov2,
                       boolean dez1, boolean dez2,
                       boolean jan21, boolean jan22) {
            this.name = new SimpleStringProperty(name);

            // Initialisierung aller Werte
            janCheck1.set(jan1); janCheck2.set(jan2);
            febCheck1.set(feb1); febCheck2.set(feb2);
            marCheck1.set(mar1); marCheck2.set(mar2);
            aprCheck1.set(apr1); aprCheck2.set(apr2);
            maiCheck1.set(mai1); maiCheck2.set(mai2);
            junCheck1.set(jun1); junCheck2.set(jun2);
            julCheck1.set(jul1); julCheck2.set(jul2);
            augCheck1.set(aug1); augCheck2.set(aug2);
            sepCheck1.set(sep1); sepCheck2.set(sep2);
            oktCheck1.set(okt1); oktCheck2.set(okt2);
            novCheck1.set(nov1); novCheck2.set(nov2);
            dezCheck1.set(dez1); dezCheck2.set(dez2);
            jan2Check1.set(jan21); jan2Check2.set(jan22);
        }

        // Getter-Methoden
        public StringProperty nameProperty() { return name; }
        public BooleanProperty janCheck1Property() { return janCheck1; }
        public BooleanProperty janCheck2Property() { return janCheck2; }
        public BooleanProperty febCheck1Property() { return febCheck1; }
        public BooleanProperty febCheck2Property() { return febCheck2; }
        public BooleanProperty marCheck1Property() { return marCheck1; }
        public BooleanProperty marCheck2Property() { return marCheck2; }
        public BooleanProperty aprCheck1Property() { return aprCheck1; }
        public BooleanProperty aprCheck2Property() { return aprCheck2; }
        public BooleanProperty maiCheck1Property() { return maiCheck1; }
        public BooleanProperty maiCheck2Property() { return maiCheck2; }
        public BooleanProperty junCheck1Property() { return junCheck1; }
        public BooleanProperty junCheck2Property() { return junCheck2; }
        public BooleanProperty julCheck1Property() { return julCheck1; }
        public BooleanProperty julCheck2Property() { return julCheck2; }
        public BooleanProperty augCheck1Property() { return augCheck1; }
        public BooleanProperty augCheck2Property() { return augCheck2; }
        public BooleanProperty sepCheck1Property() { return sepCheck1; }
        public BooleanProperty sepCheck2Property() { return sepCheck2; }
        public BooleanProperty oktCheck1Property() { return oktCheck1; }
        public BooleanProperty oktCheck2Property() { return oktCheck2; }
        public BooleanProperty novCheck1Property() { return novCheck1; }
        public BooleanProperty novCheck2Property() { return novCheck2; }
        public BooleanProperty dezCheck1Property() { return dezCheck1; }
        public BooleanProperty dezCheck2Property() { return dezCheck2; }
        public BooleanProperty jan2Check1Property() { return jan2Check1; }
        public BooleanProperty jan2Check2Property() { return jan2Check2; }
    }

    private static class DoubleCheckBoxCell extends TableCell<RowData, Void> {
        // Container zur Darstellung der beiden CheckBoxen
        private HBox container = new HBox(5);

        // Funktionen, um auf die jeweilige BooleanProperty zuzugreifen
        private final Function<RowData, BooleanProperty> check1Func;
        private final Function<RowData, BooleanProperty> check2Func;

        public DoubleCheckBoxCell(Function<RowData, BooleanProperty> check1Func,
                                  Function<RowData, BooleanProperty> check2Func) {
            this.check1Func = check1Func;
            this.check2Func = check2Func;
            container.setAlignment(Pos.CENTER);
            container.setPadding(new Insets(5));
        }

        @Override
        protected void updateItem(Void item, boolean empty) {
            super.updateItem(item, empty);
            // Entferne bisherige Inhalte und Bindings
            container.getChildren().clear();
            setGraphic(null);

            if (empty || getTableRow() == null || getTableRow().getItem() == null) {
                return;
            }

            // Hole die aktuelle Zeile anhand des TableView-Index
            RowData currentRow = getTableView().getItems().get(getIndex());

            // Erzeuge pro updateItem neue CheckBox-Instanzen, damit recycelte Zellen nicht "alte" Bindings behalten
            CheckBox check1 = new CheckBox();

            // Deaktiviere die Checkbox, sodass der Benutzer sie nicht ändern kann
            check1.setDisable(true);

            CheckBox check2 = new CheckBox();

            // Deaktiviere auch diese Checkbox
            check2.setDisable(true);

            check1.getStyleClass().add("lohn-check");
            check2.getStyleClass().add("zeit-check");

            // Binde die Checkboxen bidirektional an die entsprechenden Properties
            check1.selectedProperty().bindBidirectional(check1Func.apply(currentRow));
            check2.selectedProperty().bindBidirectional(check2Func.apply(currentRow));

            // Füge die Checkboxen zum Container hinzu und setze diesen als Grafik der Zelle
            container.getChildren().addAll(check1, check2);
            setGraphic(container);
        }
    }




    // Button-Handler
    public void handleclbutton2c(ActionEvent event) {
        try {
            Main.openClosePopup("/ClosePopup99.fxml");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public void handleklbutton2c(ActionEvent event) {
        try {
            Main.minimizeWindow();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public void handlebackbutton2c(ActionEvent event) {
        try {
            Main.changeScene("/LAZA2.fxml");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public void setcheckboxes() {
        lohnabrcheckbox2c.setSelected(true);
        zeitabrcheckbox2c.setSelected(true);
        lohnabrcheckbox2c.setDisable(true);
        zeitabrcheckbox2c.setDisable(true);
    }
}

/*import javafx.beans.property.StringProperty;
import javafx.event.ActionEvent;
import javafx.collections.FXCollections;
import javafx.collections.ObservableList;
import javafx.fxml.FXML;
import javafx.scene.control.CheckBox;
import javafx.scene.control.TableColumn;
import javafx.scene.control.TableView;
import javafx.scene.control.cell.PropertyValueFactory;


public class FORT3Controller {

    @FXML
    private TableView<RowData> tableView;
    @FXML
    private CheckBox lohnabrcheckbox2c;
    @FXML
    private CheckBox zeitabrcheckbox2c;

    ObservableList<RowData> data = FXCollections.observableArrayList();


    public void filltable(){
        TableColumn<RowData, String> stringColumn = new TableColumn<>("Arbeitnehmer");
        stringColumn.setPrefWidth(400);
        stringColumn.setCellValueFactory(new PropertyValueFactory<>("stringValue"));
        TableColumn<RowData, Boolean> check1Column = new TableColumn<>("JAN");
        check1Column.setPrefWidth(100);
        //check1Column.setCellValueFactory(new PropertyValueFactory<>("check1"));
        TableColumn<RowData, Boolean> check2Column = new TableColumn<>("FEB");
        check2Column.setPrefWidth(100);
        //check2Column.setCellValueFactory(new PropertyValueFactory<>("check2"));
        TableColumn<RowData, Boolean> check3Column = new TableColumn<>("MÄR");
        check3Column.setPrefWidth(100);
        TableColumn<RowData, Boolean> check4Column = new TableColumn<>("APR");
        check4Column.setPrefWidth(100);
        TableColumn<RowData, Boolean> check5Column = new TableColumn<>("MAI");
        check5Column.setPrefWidth(100);
        TableColumn<RowData, Boolean> check6Column = new TableColumn<>("JUN");
        check6Column.setPrefWidth(100);
        TableColumn<RowData, Boolean> check7Column = new TableColumn<>("JUL");
        check7Column.setPrefWidth(100);
        TableColumn<RowData, Boolean> check8Column = new TableColumn<>("AUG");
        check8Column.setPrefWidth(100);
        TableColumn<RowData, Boolean> check9Column = new TableColumn<>("SEP");
        check9Column.setPrefWidth(100);
        TableColumn<RowData, Boolean> check10Column = new TableColumn<>("OKT");
        check10Column.setPrefWidth(100);
        TableColumn<RowData, Boolean> check11Column = new TableColumn<>("NOV");
        check11Column.setPrefWidth(100);
        TableColumn<RowData, Boolean> check12Column = new TableColumn<>("DEZ");
        check12Column.setPrefWidth(100);
        TableColumn<RowData, Boolean> check13Column = new TableColumn<>("JAN");
        check13Column.setPrefWidth(100);
        tableView.getColumns().addAll(stringColumn, check1Column, check2Column, check3Column, check4Column, check5Column, check6Column, check7Column, check8Column,check9Column, check10Column, check11Column, check12Column, check13Column);
        tableView.setItems(data);
    }


    public void addrow(String name, boolean ch1, boolean ch2){
        data.add(new RowData(name, ch1, ch2));
        data.add(new RowData("1", true, false));
        data.add(new RowData("2", false, false));
        data.add(new RowData("3", true, false));
        data.add(new RowData("4", false, false));
    }

    public static class RowData {
        private final String stringValue;
        private final boolean ch1;
        private final boolean ch2;

        public RowData(String stringValue, boolean ch1, boolean ch2) {
            this.stringValue = stringValue;
            this.ch1 = ch1;
            this.ch2 = ch2;

        }

        public boolean ischeckbox1(){
            return ch1;
        }

        public boolean ischeckbox2(){
            return ch2;
        }

        public String getStringValue() {
            return stringValue;
        }
    }

    //KLick auf "x"
    public void handleclbutton2c(ActionEvent event) {
        try {
            Main.openClosePopup("/ClosePopup99.fxml");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "-"
    public void handleklbutton2c(ActionEvent event) {
        try {
            Main.minimizeWindow();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    //Klick auf "Zurück"
    public void handlebackbutton2c(ActionEvent event) {
        try {
            Main.changeScene("/LAZA2.fxml");
        }
        catch (Exception e) {
            e.printStackTrace();
        }
    }

    public void setcheckboxes(){
        lohnabrcheckbox2c.setSelected(true);
        zeitabrcheckbox2c.setSelected(true);
    }
}*/
