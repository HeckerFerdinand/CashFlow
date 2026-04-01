import com.itextpdf.text.*;
import com.itextpdf.text.pdf.*;
import com.itextpdf.text.pdf.draw.LineSeparator;
import java.io.FileOutputStream;
import java.io.IOException;
import java.text.DecimalFormat;


public class PDFjährlohnjournal {


    public void print(String id, String datum, String jahr) {
        Document document = new Document();
        document.setPageSize(PageSize.A4.rotate());
        document.setMargins(30, 30, 5, 5);
        SQLConnectionBase connection = new SQLConnectionBase();
        DecimalFormat dezimalformat = new DecimalFormat("#,##0.00");
        // 1. Basis-Pfad aus der Datenbank holen
        String baseDir = connection.getSetting("pdf_path");

// Sicherheits-Check für den Fall, dass kein Pfad gesetzt ist
        if (baseDir == null || baseDir.isEmpty()) {
            baseDir = System.getProperty("user.home") + java.io.File.separator + "Desktop";
        }

// 2. Mitarbeiter-Info für Ordner und Dateiname holen
        String persNr = connection.selectConstContent("arbeitnehmerkonstanten", "anpersonalnummer", id);
        String nachname = connection.selectConstContent("arbeitnehmerkonstanten", "annachname", id);
        String employeeFolder = persNr + " " + nachname;

// 3. Pfad zum Mitarbeiter-Ordner sicher zusammenbauen
        java.nio.file.Path fullPath = java.nio.file.Paths.get(baseDir, employeeFolder);

// 4. Ordner automatisch erstellen (falls nicht vorhanden)
        try {
            java.nio.file.Files.createDirectories(fullPath);
        } catch (java.io.IOException e) {
            e.printStackTrace();
        }

// 5. Dateiname für das Lohnjournal (fix auf den 31.12. gesetzt, wie in deinem Entwurf)
        String docname = jahr + "-12-31 " + employeeFolder + " Lohnjournal.pdf";
        String docfile = fullPath.toString() + java.io.File.separator;

        try {
            // PDF-Writer mit dem neuen dynamischen Pfad starten
            PdfWriter.getInstance(document, new java.io.FileOutputStream(docfile + docname));
            document.open();

            // ... dein PDF-Inhalt folgt hier ...

            // Schriftarten festlegen
            byte[] fontBytes = getClass().getResourceAsStream("/calibri.ttf").readAllBytes();
            BaseFont baseFont = BaseFont.createFont("calibri.ttf", BaseFont.WINANSI, BaseFont.EMBEDDED, BaseFont.CACHED, fontBytes, null);
            Font headlinefat = new Font(baseFont, 14, Font.BOLD, BaseColor.BLACK);
            Font headlinenormal = new Font(baseFont, 14, Font.NORMAL, BaseColor.BLACK);
            Font fat = new Font(baseFont, 10, Font.BOLD, BaseColor.BLACK);
            Font anfont = new Font(baseFont, 12, Font.BOLD, BaseColor.BLACK);
            Font normal = new Font(baseFont, 10, Font.NORMAL, BaseColor.BLACK);
            Font small = new Font(baseFont, 8, Font.NORMAL, BaseColor.BLACK);

            //Horizontaler Strich (lang)
            LineSeparator longline = new LineSeparator();
            longline.setLineWidth(1f);
            longline.setPercentage(100);
            longline.setAlignment(Element.ALIGN_CENTER);
            document.add(new Paragraph(" "));

            //Horizontaler Strich (kurz
            LineSeparator shortline = new LineSeparator();
            shortline.setLineWidth(1f);
            shortline.setPercentage(25);
            shortline.setAlignment(Element.ALIGN_RIGHT);
            document.add(new Paragraph(" "));

            // Erste Zeile
            PdfPTable headtable = new PdfPTable(2);
            headtable.setWidthPercentage(100);
            headtable.setWidths(new float[]{10, 5});
            String title = "Lohnjournal der Brutto/ Netto-Bezüge ".concat(jahr);
            headtable.addCell(createCell(title, headlinefat, Element.ALIGN_LEFT, 0, 0, 25, false));
            headtable.addCell(createCell(datum, normal, Element.ALIGN_RIGHT, 0, 0, 0, false));

            //Atrribute0.1
            PdfPTable attributeTable01 = new PdfPTable(1);
            attributeTable01.setWidthPercentage(100);
            attributeTable01.setWidths(new float[]{1});
            attributeTable01.addCell(createCell("Arbeitnehmer", fat, Element.ALIGN_LEFT, 0, 0, 5, false));

            //Atrribute1
            PdfPTable attributeTable = new PdfPTable(4);
            attributeTable.setWidthPercentage(100);
            attributeTable.setWidths(new float[]{1, 1, 1, 1});
            PdfPCell cell4 = createCell("Name", fat, Element.ALIGN_LEFT, 10, 3, 3, true);
            BaseColor color1 = new BaseColor(211, 211, 211, 225);
            cell4.setBackgroundColor(color1);
            attributeTable.addCell(cell4);
            PdfPCell cell5 = createCell("Anschrift", fat, Element.ALIGN_LEFT, 10, 3, 3, true);
            cell5.setBackgroundColor(color1);
            attributeTable.addCell(cell5);
            PdfPCell cell6 = createCell("SV-Nummer", fat, Element.ALIGN_LEFT, 10, 3, 3, true);
            cell6.setBackgroundColor(color1);
            attributeTable.addCell(cell6);
            PdfPCell cell7 = createCell("Personal-Nummer", fat, Element.ALIGN_LEFT, 10, 3, 3, false);
            cell7.setBackgroundColor(color1);
            attributeTable.addCell(cell7);
            String anname = connection.selectConstContent("arbeitnehmerkonstanten", "anvorname", id).concat(" ").concat(connection.selectConstContent("arbeitnehmerkonstanten", "annachname", id));
            attributeTable.addCell(createCell(anname, normal, Element.ALIGN_LEFT, 10, 3, 3, true));
            String ananschrift1 = connection.selectConstContent("arbeitnehmerkonstanten", "anstraße", id).concat(" ").concat(connection.selectConstContent("arbeitnehmerkonstanten", "anhausnummer", id));
            attributeTable.addCell(createCell(ananschrift1, normal, Element.ALIGN_LEFT, 10, 3, 3, true));
            String persnr = connection.selectConstContent("arbeitnehmerkonstanten", "anpersonalnummer", id);
            attributeTable.addCell(createCell(persnr, normal, Element.ALIGN_LEFT, 10, 3, 3, true));
            String bgr = connection.selectConstContent("arbeitnehmerkonstanten", "anbgrschluessel", id);
            attributeTable.addCell(createCell(bgr, normal, Element.ALIGN_LEFT, 10, 3, 3, false));
            attributeTable.addCell(createCell(" ", normal, Element.ALIGN_LEFT, 10, 3, 3, true));
            String ananschrift2 = connection.selectConstContent("arbeitnehmerkonstanten", "anpostleitzahl", id).concat(" ").concat(connection.selectConstContent("arbeitnehmerkonstanten", "anort", id));
            attributeTable.addCell(createCell(ananschrift2, normal, Element.ALIGN_LEFT, 10, 3, 3, true));
            attributeTable.addCell(createCell(" ", normal, Element.ALIGN_LEFT, 10, 3, 3, true));
            attributeTable.addCell(createCell(" ", normal, Element.ALIGN_LEFT, 10, 3, 3, false));

            //Atrribute02
            PdfPTable attributeTable02 = new PdfPTable(1);
            attributeTable02.setWidthPercentage(100);
            attributeTable02.setWidths(new float[]{1});
            attributeTable02.addCell(createCell("Arbeitgeber", fat, Element.ALIGN_LEFT, 0, 0, 5, false));

            //Atrribute2
            PdfPTable attributeTable2 = new PdfPTable(4);
            attributeTable2.setWidthPercentage(100);
            attributeTable2.setWidths(new float[]{1, 1, 1, 1});
            PdfPCell cell8 = createCell("Name", fat, Element.ALIGN_LEFT, 10, 3, 3, true);
            cell8.setBackgroundColor(color1);
            attributeTable2.addCell(cell8);
            PdfPCell cell9 = createCell("Anschrift", fat, Element.ALIGN_LEFT, 10, 3, 3, true);
            cell9.setBackgroundColor(color1);
            attributeTable2.addCell(cell9);
            PdfPCell cell10 = createCell("Betriebsnummer", fat, Element.ALIGN_LEFT, 10, 3, 3, true);
            cell10.setBackgroundColor(color1);
            attributeTable2.addCell(cell10);
            PdfPCell cell11 = createCell("Steuernummer", fat, Element.ALIGN_LEFT, 10, 3, 3, false);
            cell11.setBackgroundColor(color1);
            attributeTable2.addCell(cell11);
            String agname = connection.selectConstContent("arbeitnehmerkonstanten", "agname", id);
            attributeTable2.addCell(createCell(agname, normal, Element.ALIGN_LEFT, 10, 3, 3, true));
            String aganschrift1 = connection.selectConstContent("arbeitnehmerkonstanten", "agstraße", id).concat(" ").concat(connection.selectConstContent("arbeitnehmerkonstanten", "aghausnummer", id));
            attributeTable2.addCell(createCell(aganschrift1, normal, Element.ALIGN_LEFT, 10, 3, 3, true));
            String betrnr = connection.selectConstContent("arbeitnehmerkonstanten", "agbetriebsnummer", id);
            attributeTable2.addCell(createCell(betrnr, normal, Element.ALIGN_LEFT, 10, 3, 3, true));
            String stnr = connection.selectConstContent("arbeitnehmerkonstanten", "agsteuernummer", id);
            attributeTable2.addCell(createCell(stnr, normal, Element.ALIGN_LEFT, 10, 3, 3, false));
            String vertr;
            if (connection.selectConstContent("arbeitnehmerkonstanten", "agname2", id) != null){
               vertr = connection.selectConstContent("arbeitnehmerkonstanten", "agname2", id);
            }
            else{
                vertr = " ";
            }
            attributeTable2.addCell(createCell(vertr, normal, Element.ALIGN_LEFT, 10, 3, 3, true));
            String aganschrift2 = connection.selectConstContent("arbeitnehmerkonstanten", "agpostleitzahl", id).concat(" ").concat(connection.selectConstContent("arbeitnehmerkonstanten", "agort", id));
            attributeTable2.addCell(createCell(aganschrift2, normal, Element.ALIGN_LEFT, 10, 3, 3, true));
            attributeTable2.addCell(createCell(" ", normal, Element.ALIGN_LEFT, 10, 3, 3, true));
            attributeTable2.addCell(createCell(" ", normal, Element.ALIGN_LEFT, 10, 3, 3, false));

            //MonthTable
            PdfPTable monthTable = new PdfPTable(14);
            monthTable.setWidthPercentage(100);
            monthTable.setWidths(new float[]{4,2,2,2,2,2,2,2,2,2,2,2,2,3});
            monthTable.addCell(createCell(" ", fat, Element.ALIGN_CENTER, 10, 3, 10, false));
            monthTable.addCell(createCell("JAN", fat, Element.ALIGN_CENTER, 10, 3, 3, false));
            monthTable.addCell(createCell("FEB", fat, Element.ALIGN_CENTER, 10, 3, 3, false));
            monthTable.addCell(createCell("MÄR", fat, Element.ALIGN_CENTER, 10, 3, 3, false));
            monthTable.addCell(createCell("APR", fat, Element.ALIGN_CENTER, 10, 3, 3, false));
            monthTable.addCell(createCell("MAI", fat, Element.ALIGN_CENTER, 10, 3, 3, false));
            monthTable.addCell(createCell("JUN", fat, Element.ALIGN_CENTER, 10, 3, 3, false));
            monthTable.addCell(createCell("JUL", fat, Element.ALIGN_CENTER, 10, 3, 3, false));
            monthTable.addCell(createCell("AUG", fat, Element.ALIGN_CENTER, 10, 3, 3, false));
            monthTable.addCell(createCell("SEP", fat, Element.ALIGN_CENTER, 10, 3, 3, false));
            monthTable.addCell(createCell("OKT", fat, Element.ALIGN_CENTER, 10, 3, 3, false));
            monthTable.addCell(createCell("NOV", fat, Element.ALIGN_CENTER, 10, 3, 3, false));
            monthTable.addCell(createCell("DEZ", fat, Element.ALIGN_CENTER, 10, 3, 3, false));
            monthTable.addCell(createCell("Gesamt", fat, Element.ALIGN_RIGHT, 10, 3, 3, false));

            //MonthTable
            PdfPTable bruttotable = new PdfPTable(14);
            bruttotable.setWidthPercentage(100);
            bruttotable.setWidths(new float[]{4,2,2,2,2,2,2,2,2,2,2,2,2,3});
            //"Vergütung brutto"
            bruttotable.addCell(createCell("Vergütung brutto", normal, Element.ALIGN_LEFT, 10, 10, 3, false));
            String svb1 = connection.selectmtlContent(id, "verguetungbrutto", "Januar", jahr);
            double dvb1 = Double.parseDouble(svb1);
            String vb1 = dezimalformat.format(dvb1).concat(" €");
            bruttotable.addCell(createCell(vb1, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String svb2 = connection.selectmtlContent(id, "verguetungbrutto", "Februar", jahr);
            double dvb2 = Double.parseDouble(svb2);
            String vb2 = dezimalformat.format(dvb2).concat(" €");
            bruttotable.addCell(createCell(vb2, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String svb3 = connection.selectmtlContent(id, "verguetungbrutto", "März", jahr);
            double dvb3 = Double.parseDouble(svb3);
            String vb3 = dezimalformat.format(dvb3).concat(" €");
            bruttotable.addCell(createCell(vb3, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String svb4 = connection.selectmtlContent(id, "verguetungbrutto", "April", jahr);
            double dvb4 = Double.parseDouble(svb4);
            String vb4 = dezimalformat.format(dvb4).concat(" €");
            bruttotable.addCell(createCell(vb4, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String svb5 = connection.selectmtlContent(id, "verguetungbrutto", "Mai", jahr);
            double dvb5 = Double.parseDouble(svb5);
            String vb5 = dezimalformat.format(dvb5).concat(" €");
            bruttotable.addCell(createCell(vb5, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String svb6 = connection.selectmtlContent(id, "verguetungbrutto", "Juni", jahr);
            double dvb6 = Double.parseDouble(svb6);
            String vb6 = dezimalformat.format(dvb6).concat(" €");
            bruttotable.addCell(createCell(vb6, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String svb7 = connection.selectmtlContent(id, "verguetungbrutto", "Juli", jahr);
            double dvb7 = Double.parseDouble(svb7);
            String vb7 = dezimalformat.format(dvb7).concat(" €");
            bruttotable.addCell(createCell(vb7, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String svb8 = connection.selectmtlContent(id, "verguetungbrutto", "August", jahr);
            double dvb8 = Double.parseDouble(svb8);
            String vb8 = dezimalformat.format(dvb8).concat(" €");
            bruttotable.addCell(createCell(vb8, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String svb9 = connection.selectmtlContent(id, "verguetungbrutto", "September", jahr);
            double dvb9 = Double.parseDouble(svb9);
            String vb9 = dezimalformat.format(dvb9).concat(" €");
            bruttotable.addCell(createCell(vb9, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String svb10 = connection.selectmtlContent(id, "verguetungbrutto", "Oktober", jahr);
            double dvb10 = Double.parseDouble(svb10);
            String vb10 = dezimalformat.format(dvb10).concat(" €");
            bruttotable.addCell(createCell(vb10, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String svb11 = connection.selectmtlContent(id, "verguetungbrutto", "November", jahr);
            double dvb11 = Double.parseDouble(svb11);
            String vb11 = dezimalformat.format(dvb11).concat(" €");
            bruttotable.addCell(createCell(vb11, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String svb12 = connection.selectmtlContent(id, "verguetungbrutto", "Dezember", jahr);
            double dvb12 = Double.parseDouble(svb12);
            String vb12 = dezimalformat.format(dvb12).concat(" €");
            bruttotable.addCell(createCell(vb12, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            double doublevbgesamt = dvb1 + dvb2 + dvb3 + dvb4 + dvb5 + dvb6 + dvb7 + dvb8 + dvb9 + dvb10 + dvb11 + dvb12;
            String vbgesamt = dezimalformat.format(doublevbgesamt).concat(" €");
            bruttotable.addCell(createCell(vbgesamt, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            //"Sonstige Vergütung"
            bruttotable.addCell(createCell("Sonstige Vergütung", normal, Element.ALIGN_LEFT, 10, 5, 3, false));
            String ssvb1 = connection.selectmtlContent(id, "sonderverguetungbrutto", "Januar", jahr);
            double dsvb1 = Double.parseDouble(ssvb1);
            String vsb1 = dezimalformat.format(dsvb1).concat(" €");
            bruttotable.addCell(createCell(vsb1, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String ssvb2 = connection.selectmtlContent(id, "sonderverguetungbrutto", "Februar", jahr);
            double dsvb2 = Double.parseDouble(ssvb2);
            String vsb2 = dezimalformat.format(dsvb2).concat(" €");
            bruttotable.addCell(createCell(vsb2, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String ssvb3 = connection.selectmtlContent(id, "sonderverguetungbrutto", "März", jahr);
            double dsvb3 = Double.parseDouble(ssvb3);
            String vsb3 = dezimalformat.format(dsvb3).concat(" €");
            bruttotable.addCell(createCell(vsb3, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String ssvb4 = connection.selectmtlContent(id, "sonderverguetungbrutto", "April", jahr);
            double dsvb4 = Double.parseDouble(ssvb4);
            String vsb4 = dezimalformat.format(dsvb4).concat(" €");
            bruttotable.addCell(createCell(vsb4, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String ssvb5 = connection.selectmtlContent(id, "sonderverguetungbrutto", "Mai", jahr);
            double dsvb5 = Double.parseDouble(ssvb5);
            String vsb5 = dezimalformat.format(dsvb5).concat(" €");
            bruttotable.addCell(createCell(vsb5, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String ssvb6 = connection.selectmtlContent(id, "sonderverguetungbrutto", "Juni", jahr);
            double dsvb6 = Double.parseDouble(ssvb6);
            String vsb6 = dezimalformat.format(dsvb6).concat(" €");
            bruttotable.addCell(createCell(vsb6, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String ssvb7 = connection.selectmtlContent(id, "sonderverguetungbrutto", "Juli", jahr);
            double dsvb7 = Double.parseDouble(ssvb7);
            String vsb7 = dezimalformat.format(dsvb7).concat(" €");
            bruttotable.addCell(createCell(vsb7, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String ssvb8 = connection.selectmtlContent(id, "sonderverguetungbrutto", "August", jahr);
            double dsvb8 = Double.parseDouble(ssvb8);
            String vsb8 = dezimalformat.format(dsvb8).concat(" €");
            bruttotable.addCell(createCell(vsb8, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String ssvb9 = connection.selectmtlContent(id, "sonderverguetungbrutto", "September", jahr);
            double dsvb9 = Double.parseDouble(ssvb9);
            String vsb9 = dezimalformat.format(dsvb9).concat(" €");
            bruttotable.addCell(createCell(vsb9, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String ssvb10 = connection.selectmtlContent(id, "sonderverguetungbrutto", "Oktober", jahr);
            double dsvb10 = Double.parseDouble(ssvb10);
            String vsb10 = dezimalformat.format(dsvb10).concat(" €");
            bruttotable.addCell(createCell(vsb10, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String ssvb11 = connection.selectmtlContent(id, "sonderverguetungbrutto", "November", jahr);
            double dsvb11 = Double.parseDouble(ssvb11);
            String vsb11 = dezimalformat.format(dsvb11).concat(" €");
            bruttotable.addCell(createCell(vsb11, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String ssvb12 = connection.selectmtlContent(id, "sonderverguetungbrutto", "Dezember", jahr);
            double dsvb12 = Double.parseDouble(ssvb12);
            String vsb12 = dezimalformat.format(dsvb12).concat(" €");
            bruttotable.addCell(createCell(vsb12, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double doublesvbgesamt = dsvb1 + dsvb2 + dsvb3 + dsvb4 + dsvb5 + dsvb6 + dsvb7 + dsvb8 + dvb9 + dsvb10 + dsvb11 + dsvb12;
            String vsbgesamt = dezimalformat.format(doublesvbgesamt).concat(" €");
            bruttotable.addCell(createCell(vsbgesamt, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            //"Gesamtbrutto"
            bruttotable.addCell(createCell("Gesamtbrutto", fat, Element.ALIGN_LEFT, 10, 5, 3, false));
            String sgb1 = connection.selectmtlContent(id, "gesamtbetragbrutto", "Januar", jahr);
            double dgb1 = Double.parseDouble(sgb1);
            String gb1 = dezimalformat.format(dgb1).concat(" €");
            bruttotable.addCell(createCell(gb1, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sgb2 = connection.selectmtlContent(id, "gesamtbetragbrutto", "Februar", jahr);
            double dgb2 = Double.parseDouble(sgb2);
            String gb2 = dezimalformat.format(dgb2).concat(" €");
            bruttotable.addCell(createCell(gb2, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sgb3 = connection.selectmtlContent(id, "gesamtbetragbrutto", "März", jahr);
            double dgb3 = Double.parseDouble(sgb3);
            String gb3 = dezimalformat.format(dgb3).concat(" €");
            bruttotable.addCell(createCell(gb3, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sgb4 = connection.selectmtlContent(id, "gesamtbetragbrutto", "April", jahr);
            double dgb4 = Double.parseDouble(sgb4);
            String gb4 = dezimalformat.format(dgb4).concat(" €");
            bruttotable.addCell(createCell(gb4, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sgb5 = connection.selectmtlContent(id, "gesamtbetragbrutto", "Mai", jahr);
            double dgb5 = Double.parseDouble(sgb5);
            String gb5 = dezimalformat.format(dgb5).concat(" €");
            bruttotable.addCell(createCell(gb5, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sgb6 = connection.selectmtlContent(id, "gesamtbetragbrutto", "Juni", jahr);
            double dgb6 = Double.parseDouble(sgb6);
            String gb6 = dezimalformat.format(dgb6).concat(" €");
            bruttotable.addCell(createCell(gb6, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sgb7 = connection.selectmtlContent(id, "gesamtbetragbrutto", "Juli", jahr);
            double dgb7 = Double.parseDouble(sgb7);
            String gb7 = dezimalformat.format(dgb7).concat(" €");
            bruttotable.addCell(createCell(gb7, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sgb8 = connection.selectmtlContent(id, "gesamtbetragbrutto", "August", jahr);
            double dgb8 = Double.parseDouble(sgb8);
            String gb8 = dezimalformat.format(dgb8).concat(" €");
            bruttotable.addCell(createCell(gb8, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sgb9 = connection.selectmtlContent(id, "gesamtbetragbrutto", "September", jahr);
            double dgb9 = Double.parseDouble(sgb9);
            String gb9 = dezimalformat.format(dgb9).concat(" €");
            bruttotable.addCell(createCell(gb9, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sgb10 = connection.selectmtlContent(id, "gesamtbetragbrutto", "Oktober", jahr);
            double dgb10 = Double.parseDouble(sgb10);
            String gb10 = dezimalformat.format(dgb10).concat(" €");
            bruttotable.addCell(createCell(gb10, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sgb11 = connection.selectmtlContent(id, "gesamtbetragbrutto", "November", jahr);
            double dgb11 = Double.parseDouble(sgb11);
            String gb11 = dezimalformat.format(dgb11).concat(" €");
            bruttotable.addCell(createCell(gb11, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sgb12 = connection.selectmtlContent(id, "gesamtbetragbrutto", "Dezember", jahr);
            double dgb12 = Double.parseDouble(sgb12);
            String gb12 = dezimalformat.format(dgb12).concat(" €");
            bruttotable.addCell(createCell(gb12, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double dbggesamt = dgb1 + dgb2 + dgb3 + dgb4 + dgb5 + dgb6 + dgb7 + dgb8 + dgb9 + dgb10 + dgb11 + dgb12;
            String sbggesamt = dezimalformat.format(dbggesamt).concat(" €");
            bruttotable.addCell(createCell(sbggesamt, fat, Element.ALIGN_RIGHT, 10, 5, 3, false));
            //"Auszahlungsbetrag netto"
            bruttotable.addCell(createCell("Auszahlung netto", fat, Element.ALIGN_LEFT, 10, 5, 10, false));
            String sazb1 = connection.selectmtlContent(id, "auszahlungsbetragbrutto", "Januar", jahr);
            double dazb1 = Double.parseDouble(sazb1);
            String azb1 = dezimalformat.format(dazb1).concat(" €");
            bruttotable.addCell(createCell(azb1, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            String sazb2 = connection.selectmtlContent(id, "auszahlungsbetragbrutto", "Februar", jahr);
            double dazb2 = Double.parseDouble(sazb2);
            String azb2 = dezimalformat.format(dazb2).concat(" €");
            bruttotable.addCell(createCell(azb2, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            String sazb3 = connection.selectmtlContent(id, "auszahlungsbetragbrutto", "März", jahr);
            double dazb3 = Double.parseDouble(sazb3);
            String azb3 = dezimalformat.format(dazb3).concat(" €");
            bruttotable.addCell(createCell(azb3, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            String sazb4 = connection.selectmtlContent(id, "auszahlungsbetragbrutto", "April", jahr);
            double dazb4 = Double.parseDouble(sazb4);
            String azb4 = dezimalformat.format(dazb4).concat(" €");
            bruttotable.addCell(createCell(azb4, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            String sazb5 = connection.selectmtlContent(id, "auszahlungsbetragbrutto", "Mai", jahr);
            double dazb5 = Double.parseDouble(sazb5);
            String azb5 = dezimalformat.format(dazb5).concat(" €");
            bruttotable.addCell(createCell(azb5, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            String sazb6 = connection.selectmtlContent(id, "auszahlungsbetragbrutto", "Juni", jahr);
            double dazb6 = Double.parseDouble(sazb6);
            String azb6 = dezimalformat.format(dazb6).concat(" €");
            bruttotable.addCell(createCell(azb6, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            String sazb7 = connection.selectmtlContent(id, "auszahlungsbetragbrutto", "Juli", jahr);
            double dazb7 = Double.parseDouble(sazb7);
            String azb7 = dezimalformat.format(dazb7).concat(" €");
            bruttotable.addCell(createCell(azb7, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            String sazb8 = connection.selectmtlContent(id, "auszahlungsbetragbrutto", "August", jahr);
            double dazb8 = Double.parseDouble(sazb8);
            String azb8 = dezimalformat.format(dazb8).concat(" €");
            bruttotable.addCell(createCell(azb8, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            String sazb9 = connection.selectmtlContent(id, "auszahlungsbetragbrutto", "September", jahr);
            double dazb9 = Double.parseDouble(sazb9);
            String azb9 = dezimalformat.format(dazb9).concat(" €");
            bruttotable.addCell(createCell(azb9, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            String sazb10 = connection.selectmtlContent(id, "auszahlungsbetragbrutto", "Oktober", jahr);
            double dazb10 = Double.parseDouble(sazb10);
            String azb10 = dezimalformat.format(dazb10).concat(" €");
            bruttotable.addCell(createCell(azb10, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            String sazb11 = connection.selectmtlContent(id, "auszahlungsbetragbrutto", "November", jahr);
            double dazb11 = Double.parseDouble(sazb11);
            String azb11 = dezimalformat.format(dazb11).concat(" €");
            bruttotable.addCell(createCell(azb11, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            String sazb12 = connection.selectmtlContent(id, "auszahlungsbetragbrutto", "Dezember", jahr);
            double dazb12 = Double.parseDouble(sazb12);
            String azb12 = dezimalformat.format(dazb12).concat(" €");
            bruttotable.addCell(createCell(azb12, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            double dazbgesamt = dazb1 + dazb2 + dazb3 + dazb4 + dazb5 + dazb6 + dazb7 + dazb8 + dazb9 + dazb10 + dazb11 + dazb12;
            String sazbgesamt = dezimalformat.format(dazbgesamt).concat(" €");
            bruttotable.addCell(createCell(sazbgesamt, fat, Element.ALIGN_RIGHT, 10, 5, 10, false));

            //beitragTable
            PdfPTable beitragTable = new PdfPTable(14);
            beitragTable.setWidthPercentage(100);
            beitragTable.setWidths(new float[]{4,2,2,2,2,2,2,2,2,2,2,2,2,3});
            //KV-Beitrag
            beitragTable.addCell(createCell("KV-Beitrag", normal, Element.ALIGN_LEFT, 10, 10, 3, false));
            String skv1 = connection.selectmtlContent(id, "kv", "Januar", jahr);
            double dkv1 = Double.parseDouble(skv1) * dgb1;
            String kv1 = dezimalformat.format(dkv1).concat(" €");
            beitragTable.addCell(createCell(kv1, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String skv2 = connection.selectmtlContent(id, "kv", "Februar", jahr);
            double dkv2 = Double.parseDouble(skv2) * dgb2;
            String kv2 = dezimalformat.format(dkv2).concat(" €");
            beitragTable.addCell(createCell(kv2, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String skv3 = connection.selectmtlContent(id, "kv", "März", jahr);
            double dkv3 = Double.parseDouble(skv3) * dgb3;
            String kv3 = dezimalformat.format(dkv3).concat(" €");
            beitragTable.addCell(createCell(kv3, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String skv4 = connection.selectmtlContent(id, "kv", "April", jahr);
            double dkv4 = Double.parseDouble(skv4) * dgb4;
            String kv4 = dezimalformat.format(dkv4).concat(" €");
            beitragTable.addCell(createCell(kv4, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String skv5 = connection.selectmtlContent(id, "kv", "Mai", jahr);
            double dkv5 = Double.parseDouble(skv5) * dgb5;
            String kv5 = dezimalformat.format(dkv5).concat(" €");
            beitragTable.addCell(createCell(kv5, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String skv6 = connection.selectmtlContent(id, "kv", "Juni", jahr);
            double dkv6 = Double.parseDouble(skv6) * dgb6;
            String kv6 = dezimalformat.format(dkv6).concat(" €");
            beitragTable.addCell(createCell(kv6, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String skv7 = connection.selectmtlContent(id, "kv", "Juli", jahr);
            double dkv7 = Double.parseDouble(skv7) * dgb7;
            String kv7 = dezimalformat.format(dkv7).concat(" €");
            beitragTable.addCell(createCell(kv7, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String skv8 = connection.selectmtlContent(id, "kv", "August", jahr);
            double dkv8 = Double.parseDouble(skv8) * dgb8;
            String kv8 = dezimalformat.format(dkv8).concat(" €");
            beitragTable.addCell(createCell(kv8, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String skv9 = connection.selectmtlContent(id, "kv", "September", jahr);
            double dkv9 = Double.parseDouble(skv9) * dgb9;
            String kv9 = dezimalformat.format(dkv9).concat(" €");
            beitragTable.addCell(createCell(kv9, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String skv10 = connection.selectmtlContent(id, "kv", "Oktober", jahr);
            double dkv10 = Double.parseDouble(skv10) * dgb10;
            String kv10 = dezimalformat.format(dkv10).concat(" €");
            beitragTable.addCell(createCell(kv10, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String skv11 = connection.selectmtlContent(id, "kv", "November", jahr);
            double dkv11 = Double.parseDouble(skv11) * dgb11;
            String kv11 = dezimalformat.format(dkv11).concat(" €");
            beitragTable.addCell(createCell(kv11, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String skv12 = connection.selectmtlContent(id, "kv", "Dezember", jahr);
            double dkv12 = Double.parseDouble(skv12) * dgb12;
            String kv12 = dezimalformat.format(dkv12).concat(" €");
            beitragTable.addCell(createCell(kv12, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            double dkvgesamt = dkv1 + dkv2 + dkv3 + dkv4 + dkv5 + dkv6 + dkv7 + dkv8 + dkv9 + dkv10 + dkv11 + dkv12;
            String skvgesamt = dezimalformat.format(dkvgesamt).concat(" €");
            beitragTable.addCell(createCell(skvgesamt, normal, Element.ALIGN_RIGHT, 10, 10, 5, false));
            //RV-Beitrag
            beitragTable.addCell(createCell("RV-Beitrag", normal, Element.ALIGN_LEFT, 10, 5, 3, false));
            String srv1 = connection.selectmtlContent(id, "rv", "Januar", jahr);
            double drv1 = Double.parseDouble(srv1) * dgb1;
            String rv1 = dezimalformat.format(drv1).concat(" €");
            beitragTable.addCell(createCell(rv1, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String srv2 = connection.selectmtlContent(id, "rv", "Februar", jahr);
            double drv2 = Double.parseDouble(srv2) * dgb2;
            String rv2 = dezimalformat.format(drv2).concat(" €");
            beitragTable.addCell(createCell(rv2, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String srv3 = connection.selectmtlContent(id, "rv", "März", jahr);
            double drv3 = Double.parseDouble(srv3) * dgb3;
            String rv3 = dezimalformat.format(drv3).concat(" €");
            beitragTable.addCell(createCell(rv3, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String srv4 = connection.selectmtlContent(id, "rv", "April", jahr);
            double drv4 = Double.parseDouble(srv4) * dgb4;
            String rv4 = dezimalformat.format(drv4).concat(" €");
            beitragTable.addCell(createCell(rv4, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String srv5 = connection.selectmtlContent(id, "rv", "Mai", jahr);
            double drv5 = Double.parseDouble(srv5) * dgb5;
            String rv5 = dezimalformat.format(drv5).concat(" €");
            beitragTable.addCell(createCell(rv5, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String srv6 = connection.selectmtlContent(id, "rv", "Juni", jahr);
            double drv6 = Double.parseDouble(srv6) * dgb6;
            String rv6 = dezimalformat.format(drv6).concat(" €");
            beitragTable.addCell(createCell(rv6, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String srv7 = connection.selectmtlContent(id, "rv", "Juli", jahr);
            double drv7 = Double.parseDouble(srv7) * dgb7;
            String rv7 = dezimalformat.format(drv7).concat(" €");
            beitragTable.addCell(createCell(rv7, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String srv8 = connection.selectmtlContent(id, "rv", "August", jahr);
            double drv8 = Double.parseDouble(srv8) * dgb8;
            String rv8 = dezimalformat.format(drv8).concat(" €");
            beitragTable.addCell(createCell(rv8, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String srv9 = connection.selectmtlContent(id, "rv", "September", jahr);
            double drv9 = Double.parseDouble(srv9) * dgb9;
            String rv9 = dezimalformat.format(drv9).concat(" €");
            beitragTable.addCell(createCell(rv9, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String srv10 = connection.selectmtlContent(id, "rv", "Oktober", jahr);
            double drv10 = Double.parseDouble(srv10) * dgb10;
            String rv10 = dezimalformat.format(drv10).concat(" €");
            beitragTable.addCell(createCell(rv10, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String srv11 = connection.selectmtlContent(id, "rv", "November", jahr);
            double drv11 = Double.parseDouble(srv11) * dgb11;
            String rv11 = dezimalformat.format(drv11).concat(" €");
            beitragTable.addCell(createCell(rv11, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String srv12 = connection.selectmtlContent(id, "rv", "Dezember", jahr);
            double drv12 = Double.parseDouble(srv12) * dgb12;
            String rv12 = dezimalformat.format(drv12).concat(" €");
            beitragTable.addCell(createCell(rv12, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double drvgesamt = drv1 + drv2 + drv3 + drv4 + drv5 + drv6 + drv7 + drv8 + drv9 + drv10 + drv11 + drv12;
            String srvgesamt = dezimalformat.format(drvgesamt).concat(" €");
            beitragTable.addCell(createCell(srvgesamt, normal, Element.ALIGN_RIGHT, 10, 5, 5, false));
            //U1-Beitrag
            beitragTable.addCell(createCell("Umlage U1", normal, Element.ALIGN_LEFT, 10, 5, 3, false));
            String su11 = connection.selectmtlContent(id, "u1", "Januar", jahr);
            double du11 = Double.parseDouble(su11) * dgb1;
            String u11 = dezimalformat.format(du11).concat(" €");
            beitragTable.addCell(createCell(u11, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String su12 = connection.selectmtlContent(id, "u1", "Februar", jahr);
            double du12 = Double.parseDouble(su12) * dgb2;
            String u12 = dezimalformat.format(du12).concat(" €");
            beitragTable.addCell(createCell(u12, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String su13 = connection.selectmtlContent(id, "u1", "März", jahr);
            double du13 = Double.parseDouble(su13) * dgb3;
            String u13 = dezimalformat.format(du13).concat(" €");
            beitragTable.addCell(createCell(u13, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String su14 = connection.selectmtlContent(id, "u1", "April", jahr);
            double du14 = Double.parseDouble(su14) * dgb4;
            String u14 = dezimalformat.format(du14).concat(" €");
            beitragTable.addCell(createCell(u14, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String su15 = connection.selectmtlContent(id, "u1", "Mai", jahr);
            double du15 = Double.parseDouble(su15) * dgb5;
            String u15 = dezimalformat.format(du15).concat(" €");
            beitragTable.addCell(createCell(u15, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String su16 = connection.selectmtlContent(id, "u1", "Juni", jahr);
            double du16 = Double.parseDouble(su16) * dgb6;
            String u16 = dezimalformat.format(du16).concat(" €");
            beitragTable.addCell(createCell(u16, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String su17 = connection.selectmtlContent(id, "u1", "Juli", jahr);
            double du17 = Double.parseDouble(su17) * dgb7;
            String u17 = dezimalformat.format(du17).concat(" €");
            beitragTable.addCell(createCell(u17, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String su18 = connection.selectmtlContent(id, "u1", "August", jahr);
            double du18 = Double.parseDouble(su18) * dgb8;
            String u18 = dezimalformat.format(du18).concat(" €");
            beitragTable.addCell(createCell(u18, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String su19 = connection.selectmtlContent(id, "u1", "September", jahr);
            double du19 = Double.parseDouble(su19) * dgb9;
            String u19 = dezimalformat.format(du19).concat(" €");
            beitragTable.addCell(createCell(u19, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String su110 = connection.selectmtlContent(id, "u1", "Oktober", jahr);
            double du110 = Double.parseDouble(su110) * dgb10;
            String u110 = dezimalformat.format(du110).concat(" €");
            beitragTable.addCell(createCell(u110, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String su111 = connection.selectmtlContent(id, "u1", "November", jahr);
            double du111 = Double.parseDouble(su111) * dgb11;
            String u111 = dezimalformat.format(du111).concat(" €");
            beitragTable.addCell(createCell(u111, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String su112 = connection.selectmtlContent(id, "u1", "Dezember", jahr);
            double du112 = Double.parseDouble(su112) * dgb12;
            String u112 = dezimalformat.format(du112).concat(" €");
            beitragTable.addCell(createCell(u112, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double du1gesamt = du11 + du12 + du13 + du14 + du15 + du16 + du17 + du18 + du19 + du110 + du111 + du112;
            String su1gesamt = dezimalformat.format(du1gesamt).concat(" €");
            beitragTable.addCell(createCell(su1gesamt, normal, Element.ALIGN_RIGHT, 10, 5, 5, false));
            //U2-Beitrag
            beitragTable.addCell(createCell("Umlage U2", normal, Element.ALIGN_LEFT, 10, 5, 3, false));
            String su21 = connection.selectmtlContent(id, "u2", "Januar", jahr);
            double du21 = Double.parseDouble(su21) * dgb1;
            String u21 = dezimalformat.format(du21).concat(" €");
            beitragTable.addCell(createCell(u21, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String su22 = connection.selectmtlContent(id, "u2", "Februar", jahr);
            double du22 = Double.parseDouble(su22) * dgb2;
            String u22 = dezimalformat.format(du22).concat(" €");
            beitragTable.addCell(createCell(u22, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String su23 = connection.selectmtlContent(id, "u2", "März", jahr);
            double du23 = Double.parseDouble(su23) * dgb3;
            String u23 = dezimalformat.format(du23).concat(" €");
            beitragTable.addCell(createCell(u23, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String su24 = connection.selectmtlContent(id, "u2", "April", jahr);
            double du24 = Double.parseDouble(su24) * dgb4;
            String u24 = dezimalformat.format(du24).concat(" €");
            beitragTable.addCell(createCell(u24, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String su25 = connection.selectmtlContent(id, "u2", "Mai", jahr);
            double du25 = Double.parseDouble(su25) * dgb5;
            String u25 = dezimalformat.format(du25).concat(" €");
            beitragTable.addCell(createCell(u25, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String su26 = connection.selectmtlContent(id, "u2", "Juni", jahr);
            double du26 = Double.parseDouble(su26) * dgb6;
            String u26 = dezimalformat.format(du26).concat(" €");
            beitragTable.addCell(createCell(u26, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String su27 = connection.selectmtlContent(id, "u2", "Juli", jahr);
            double du27 = Double.parseDouble(su27) * dgb7;
            String u27 = dezimalformat.format(du27).concat(" €");
            beitragTable.addCell(createCell(u27, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String su28 = connection.selectmtlContent(id, "u2", "August", jahr);
            double du28 = Double.parseDouble(su28) * dgb8;
            String u28 = dezimalformat.format(du28).concat(" €");
            beitragTable.addCell(createCell(u28, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String su29 = connection.selectmtlContent(id, "u2", "September", jahr);
            double du29 = Double.parseDouble(su29) * dgb9;
            String u29 = dezimalformat.format(du29).concat(" €");
            beitragTable.addCell(createCell(u29, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String su210 = connection.selectmtlContent(id, "u2", "Oktober", jahr);
            double du210 = Double.parseDouble(su210) * dgb10;
            String u210 = dezimalformat.format(du210).concat(" €");
            beitragTable.addCell(createCell(u210, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String su211 = connection.selectmtlContent(id, "u2", "November", jahr);
            double du211 = Double.parseDouble(su211) * dgb11;
            String u211 = dezimalformat.format(du211).concat(" €");
            beitragTable.addCell(createCell(u211, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String su212 = connection.selectmtlContent(id, "u2", "Dezember", jahr);
            double du212 = Double.parseDouble(su212) * dgb12;
            String u212 = dezimalformat.format(du212).concat(" €");
            beitragTable.addCell(createCell(u212, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double du2gesamt = du21 + du22 + du23 + du24 + du25 + du26 + du27 + du28 + du29 + du210 + du211 + du212;
            String su2gesamt = dezimalformat.format(du2gesamt).concat(" €");
            beitragTable.addCell(createCell(su2gesamt, normal, Element.ALIGN_RIGHT, 10, 5, 5, false));
            //INSO-Beitrag
            beitragTable.addCell(createCell("Umlage Insolvenz", normal, Element.ALIGN_LEFT, 10, 5, 3, false));
            String sinso1 = connection.selectmtlContent(id, "inso", "Januar", jahr);
            double dinso1 = Double.parseDouble(sinso1) * dgb1;
            String inso1 = dezimalformat.format(dinso1).concat(" €");
            beitragTable.addCell(createCell(inso1, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sinso2 = connection.selectmtlContent(id, "inso", "Februar", jahr);
            double dinso2 = Double.parseDouble(sinso2) * dgb2;
            String inso2 = dezimalformat.format(dinso2).concat(" €");
            beitragTable.addCell(createCell(inso2, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sinso3 = connection.selectmtlContent(id, "inso", "März", jahr);
            double dinso3 = Double.parseDouble(sinso3) * dgb3;
            String inso3 = dezimalformat.format(dinso3).concat(" €");
            beitragTable.addCell(createCell(inso3, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sinso4 = connection.selectmtlContent(id, "inso", "April", jahr);
            double dinso4 = Double.parseDouble(sinso4) * dgb4;
            String inso4 = dezimalformat.format(dinso4).concat(" €");
            beitragTable.addCell(createCell(inso4, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sinso5 = connection.selectmtlContent(id, "inso", "Mai", jahr);
            double dinso5 = Double.parseDouble(sinso5) * dgb5;
            String inso5 = dezimalformat.format(dinso5).concat(" €");
            beitragTable.addCell(createCell(inso5, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sinso6 = connection.selectmtlContent(id, "inso", "Juni", jahr);
            double dinso6 = Double.parseDouble(sinso6) * dgb6;
            String inso6 = dezimalformat.format(dinso6).concat(" €");
            beitragTable.addCell(createCell(inso6, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sinso7 = connection.selectmtlContent(id, "inso", "Juli", jahr);
            double dinso7 = Double.parseDouble(sinso7) * dgb7;
            String inso7 = dezimalformat.format(dinso7).concat(" €");
            beitragTable.addCell(createCell(inso7, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sinso8 = connection.selectmtlContent(id, "inso", "August", jahr);
            double dinso8 = Double.parseDouble(sinso8) * dgb8;
            String inso8 = dezimalformat.format(dinso8).concat(" €");
            beitragTable.addCell(createCell(inso8, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sinso9 = connection.selectmtlContent(id, "inso", "September", jahr);
            double dinso9 = Double.parseDouble(sinso9) * dgb9;
            String inso9 = dezimalformat.format(dinso9).concat(" €");
            beitragTable.addCell(createCell(inso9, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sinso10 = connection.selectmtlContent(id, "inso", "Oktober", jahr);
            double dinso10 = Double.parseDouble(sinso10) * dgb10;
            String inso10 = dezimalformat.format(dinso10).concat(" €");
            beitragTable.addCell(createCell(inso10, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sinso11 = connection.selectmtlContent(id, "inso", "November", jahr);
            double dinso11 = Double.parseDouble(sinso11) * dgb11;
            String inso11 = dezimalformat.format(dinso11).concat(" €");
            beitragTable.addCell(createCell(inso11, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sinso12 = connection.selectmtlContent(id, "inso", "Dezember", jahr);
            double dinso12 = Double.parseDouble(sinso12) * dgb12;
            String inso12 = dezimalformat.format(dinso12).concat(" €");
            beitragTable.addCell(createCell(inso12, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double dinsogesamt = dinso1 + dinso2 + dinso3 + dinso4 + dinso5 + dinso6 + dinso7 + dinso8 + dinso9 + dinso10 + dinso11 + dinso12;
            String sinsogesamt = dezimalformat.format(dinsogesamt).concat(" €");
            beitragTable.addCell(createCell(sinsogesamt, normal, Element.ALIGN_RIGHT, 10, 5, 5, false));
            //ST-Beitrag
            beitragTable.addCell(createCell("Pauschalsteuer ST", normal, Element.ALIGN_LEFT, 10, 5, 10, false));
            String sst1 = connection.selectmtlContent(id, "st", "Januar", jahr);
            double dst1 = Double.parseDouble(sst1) * dgb1;
            String st1 = dezimalformat.format(dst1).concat(" €");
            beitragTable.addCell(createCell(st1, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            String sst2 = connection.selectmtlContent(id, "st", "Februar", jahr);
            double dst2 = Double.parseDouble(sst2) * dgb2;
            String st2 = dezimalformat.format(dst2).concat(" €");
            beitragTable.addCell(createCell(st2, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            String sst3 = connection.selectmtlContent(id, "st", "März", jahr);
            double dst3 = Double.parseDouble(sst3) * dgb3;
            String st3 = dezimalformat.format(dst3).concat(" €");
            beitragTable.addCell(createCell(st3, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            String sst4 = connection.selectmtlContent(id, "st", "April", jahr);
            double dst4 = Double.parseDouble(sst4) * dgb4;
            String st4 = dezimalformat.format(dst4).concat(" €");
            beitragTable.addCell(createCell(st4, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            String sst5 = connection.selectmtlContent(id, "st", "Mai", jahr);
            double dst5 = Double.parseDouble(sst5) * dgb5;
            String st5 = dezimalformat.format(dst5).concat(" €");
            beitragTable.addCell(createCell(st5, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            String sst6 = connection.selectmtlContent(id, "st", "Juni", jahr);
            double dst6 = Double.parseDouble(sst6) * dgb6;
            String st6 = dezimalformat.format(dst6).concat(" €");
            beitragTable.addCell(createCell(st6, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            String sst7 = connection.selectmtlContent(id, "st", "Juli", jahr);
            double dst7 = Double.parseDouble(sst7) * dgb7;
            String st7 = dezimalformat.format(dst7).concat(" €");
            beitragTable.addCell(createCell(st7, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            String sst8 = connection.selectmtlContent(id, "st", "August", jahr);
            double dst8 = Double.parseDouble(sst8) * dgb8;
            String st8 = dezimalformat.format(dst8).concat(" €");
            beitragTable.addCell(createCell(st8, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            String sst9 = connection.selectmtlContent(id, "st", "September", jahr);
            double dst9 = Double.parseDouble(sst9) * dgb9;
            String st9 = dezimalformat.format(dst9).concat(" €");
            beitragTable.addCell(createCell(st9, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            String sst10 = connection.selectmtlContent(id, "st", "Oktober", jahr);
            double dst10 = Double.parseDouble(sst10) * dgb10;
            String st10 = dezimalformat.format(dst10).concat(" €");
            beitragTable.addCell(createCell(st10, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            String sst11 = connection.selectmtlContent(id, "st", "November", jahr);
            double dst11 = Double.parseDouble(sst11) * dgb11;
            String st11 = dezimalformat.format(dst11).concat(" €");
            beitragTable.addCell(createCell(st11, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            String sst12 = connection.selectmtlContent(id, "st", "Dezember", jahr);
            double dst12 = Double.parseDouble(sst12) * dgb12;
            String st12 = dezimalformat.format(dst12).concat(" €");
            beitragTable.addCell(createCell(st12, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            double dstgesamt = dst1 + dst2 + dst3 + dst4 + dst5 + dst6 + dst7 + dst8 + dst9 + dst10 + dst11 + dst12;
            String sstgesamt = dezimalformat.format(dstgesamt).concat(" €");
            beitragTable.addCell(createCell(sstgesamt, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));

            //GesamtbeitragTable
            PdfPTable gesamtbeitragTable = new PdfPTable(14);
            gesamtbeitragTable.setWidthPercentage(100);
            gesamtbeitragTable.setWidths(new float[]{4,2,2,2,2,2,2,2,2,2,2,2,2,3});
            gesamtbeitragTable.addCell(createCell("Gesamtbeitrag", fat, Element.ALIGN_LEFT, 10, 5, 3, false));
            String sgesamtbeitrag1 = connection.selectmtlContent(id, "gesamtbeitrag", "Januar", jahr);
            double dgesamtbeitrag1 = Double.parseDouble(sgesamtbeitrag1);
            String gesamtbeitrag1 = dezimalformat.format(dgesamtbeitrag1).concat(" €");
            gesamtbeitragTable.addCell(createCell(gesamtbeitrag1, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sgesamtbeitrag2 = connection.selectmtlContent(id, "gesamtbeitrag", "Februar", jahr);
            double dgesamtbeitrag2 = Double.parseDouble(sgesamtbeitrag2);
            String gesamtbeitrag2 = dezimalformat.format(dgesamtbeitrag2).concat(" €");
            gesamtbeitragTable.addCell(createCell(gesamtbeitrag2, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sgesamtbeitrag3 = connection.selectmtlContent(id, "gesamtbeitrag", "März", jahr);
            double dgesamtbeitrag3 = Double.parseDouble(sgesamtbeitrag3);
            String gesamtbeitrag3 = dezimalformat.format(dgesamtbeitrag3).concat(" €");
            gesamtbeitragTable.addCell(createCell(gesamtbeitrag3, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sgesamtbeitrag4 = connection.selectmtlContent(id, "gesamtbeitrag", "April", jahr);
            double dgesamtbeitrag4 = Double.parseDouble(sgesamtbeitrag4);
            String gesamtbeitrag4 = dezimalformat.format(dgesamtbeitrag4).concat(" €");
            gesamtbeitragTable.addCell(createCell(gesamtbeitrag4, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sgesamtbeitrag5 = connection.selectmtlContent(id, "gesamtbeitrag", "Mai", jahr);
            double dgesamtbeitrag5 = Double.parseDouble(sgesamtbeitrag5);
            String gesamtbeitrag5 = dezimalformat.format(dgesamtbeitrag5).concat(" €");
            gesamtbeitragTable.addCell(createCell(gesamtbeitrag5, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sgesamtbeitrag6 = connection.selectmtlContent(id, "gesamtbeitrag", "Juni", jahr);
            double dgesamtbeitrag6 = Double.parseDouble(sgesamtbeitrag6);
            String gesamtbeitrag6 = dezimalformat.format(dgesamtbeitrag6).concat(" €");
            gesamtbeitragTable.addCell(createCell(gesamtbeitrag6, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sgesamtbeitrag7 = connection.selectmtlContent(id, "gesamtbeitrag", "Juli", jahr);
            double dgesamtbeitrag7 = Double.parseDouble(sgesamtbeitrag7);
            String gesamtbeitrag7 = dezimalformat.format(dgesamtbeitrag7).concat(" €");
            gesamtbeitragTable.addCell(createCell(gesamtbeitrag7, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sgesamtbeitrag8 = connection.selectmtlContent(id, "gesamtbeitrag", "August", jahr);
            double dgesamtbeitrag8 = Double.parseDouble(sgesamtbeitrag8);
            String gesamtbeitrag8 = dezimalformat.format(dgesamtbeitrag8).concat(" €");
            gesamtbeitragTable.addCell(createCell(gesamtbeitrag8, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sgesamtbeitrag9 = connection.selectmtlContent(id, "gesamtbeitrag", "September", jahr);
            double dgesamtbeitrag9 = Double.parseDouble(sgesamtbeitrag9);
            String gesamtbeitrag9 = dezimalformat.format(dgesamtbeitrag9).concat(" €");
            gesamtbeitragTable.addCell(createCell(gesamtbeitrag9, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sgesamtbeitrag10 = connection.selectmtlContent(id, "gesamtbeitrag", "Oktober", jahr);
            double dgesamtbeitrag10 = Double.parseDouble(sgesamtbeitrag10);
            String gesamtbeitrag10 = dezimalformat.format(dgesamtbeitrag10).concat(" €");
            gesamtbeitragTable.addCell(createCell(gesamtbeitrag10, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sgesamtbeitrag11 = connection.selectmtlContent(id, "gesamtbeitrag", "November", jahr);
            double dgesamtbeitrag11 = Double.parseDouble(sgesamtbeitrag11);
            String gesamtbeitrag11 = dezimalformat.format(dgesamtbeitrag11).concat(" €");
            gesamtbeitragTable.addCell(createCell(gesamtbeitrag11, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String sgesamtbeitrag12 = connection.selectmtlContent(id, "gesamtbeitrag", "Dezember", jahr);
            double dgesamtbeitrag12 = Double.parseDouble(sgesamtbeitrag12);
            String gesamtbeitrag12 = dezimalformat.format(dgesamtbeitrag12).concat(" €");
            gesamtbeitragTable.addCell(createCell(gesamtbeitrag12, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double dgesamtbeitraggesamt = dgesamtbeitrag1 + dgesamtbeitrag2 + dgesamtbeitrag3 + dgesamtbeitrag4 + dgesamtbeitrag5 + dgesamtbeitrag6 + dgesamtbeitrag7 + dgesamtbeitrag8 + dgesamtbeitrag9 + dgesamtbeitrag10 + dgesamtbeitrag11 + dgesamtbeitrag12;
            String sgesamtbeitraggesamt = dezimalformat.format(dgesamtbeitraggesamt).concat(" €");
            gesamtbeitragTable.addCell(createCell(sgesamtbeitraggesamt, fat, Element.ALIGN_RIGHT, 10, 5, 5, false));

            //KleingedrucktesTabelle
            PdfPTable kleingedrucktesTable = new PdfPTable(2);
            kleingedrucktesTable.setWidthPercentage(100);
            kleingedrucktesTable.setWidths(new float[]{1, 1});
            kleingedrucktesTable.addCell(createCell("", small, Element.ALIGN_LEFT, 0, 5, 5, false));
            PdfPCell imageCell = new PdfPCell();
            imageCell.setBorder(Rectangle.NO_BORDER);
            java.net.URL logoUrl = getClass().getResource("/Logo1-removebg.png");

            if (logoUrl != null) {
                Image img = Image.getInstance(logoUrl);
                img.scaleToFit(30, 30);
                img.setAlignment(Element.ALIGN_RIGHT);
                imageCell.addElement(img);
            } else {
                System.err.println("Logo konnte nicht gefunden werden! Pfad prüfen.");
            }
            imageCell.setPaddingRight(6);
            kleingedrucktesTable.addCell(imageCell);
            kleingedrucktesTable.addCell(createCell("Dieses Dokument wurde von CashFlow generiert.", small, Element.ALIGN_LEFT, 0, 5, 5, false));
            kleingedrucktesTable.addCell(createCell("CashFlow", fat, Element.ALIGN_RIGHT, 0, 5, 5, false));


            document.add(headtable);
            document.add(attributeTable01);
            document.add(longline);
            document.add(attributeTable);
            document.add(attributeTable02);
            document.add(longline);
            document.add(attributeTable2);
            document.add(new Paragraph(" "));
            document.add(monthTable);
            document.add(longline);
            document.add(bruttotable);
            document.add(longline);
            document.add(beitragTable);
            document.add(longline);
            document.add(gesamtbeitragTable);
            document.add(new Paragraph(" "));
            document.add(new Paragraph(" "));
            document.add(kleingedrucktesTable);


            document.close();
            CONFIRMPopup confirmPopup = new CONFIRMPopup();
            confirmPopup.display("CONFIRM99.fxml");
            }
        catch (DocumentException | IOException e) {
            e.printStackTrace();
            try {
                Main.changeScene("/LAZA2.fxml");
                CONFIRMPopup confirmPopup = new CONFIRMPopup();
                confirmPopup.display("NOTCONFIRM99.fxml");
            } catch (Exception e1) {
                e1.printStackTrace();
            }
        }
    }

    private PdfPCell createCell (String content, Font font,int alignment, float paddingLeftRight, float paddingTop, float paddingBottom, boolean rightborder) {
        PdfPCell cell = new PdfPCell(new Phrase(content, font));
        cell.setBorder(Rectangle.NO_BORDER);
        if (rightborder) {
            cell.setBorderWidthRight(1f);
        }
        cell.setHorizontalAlignment(alignment);
        cell.setPaddingRight(paddingLeftRight);
        cell.setPaddingTop(paddingTop);
        cell.setPaddingBottom(paddingBottom);
        return cell;
    }
}



