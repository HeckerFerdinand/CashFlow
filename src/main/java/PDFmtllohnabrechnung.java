import com.itextpdf.text.*;
import com.itextpdf.text.pdf.*;
import com.itextpdf.text.pdf.draw.LineSeparator;
import java.io.FileOutputStream;
import java.io.IOException;
import java.text.DecimalFormat;


public class PDFmtllohnabrechnung {


    public void print(String id, String datum, String monat, String jahr) {
        Document document = new Document();
        SQLConnectionBase connection = new SQLConnectionBase();
        DecimalFormat dezimalformat = new DecimalFormat("#,##0.00");
        String lastday = new String();
        String month = new String();
        switch (monat){
            case "Januar":{
                lastday = "31";
                month = "01";
                break;
            }
            case "Februar":{
                lastday = "28";
                month = "02";
                break;
            }
            case "März":{
                lastday = "31";
                month = "03";
                break;
            }
            case "April":{
                lastday = "30";
                month = "04";
                break;
            }
            case "Mai":{
                lastday = "31";
                month = "05";
                break;
            }
            case "Juni":{
                lastday = "30";
                month = "06";
                break;
            }
            case "Juli":{
                lastday = "31";
                month = "07";
                break;
            }
            case "August":{
                lastday = "31";
                month = "08";
                break;
            }
            case "September":{
                lastday = "30";
                month = "09";
                break;
            }
            case "Oktober":{
                lastday = "31";
                month = "10";
                break;
            }
            case "November":{
                lastday = "30";
                month = "11";
                break;
            }
            case "Dezember":{
                lastday = "31";
                month = "12";
                break;
            }
        }
        String docname = jahr + "-" + month + "-" + lastday + " " + connection.selectConstContent("arbeitnehmerkonstanten", "anpersonalnummer", id) + " " + connection.selectConstContent("arbeitnehmerkonstanten", "annachname", id) + " Lohnabrechnung.pdf";
        String docfile = "C:/Users/Anwender/IdeaProjects/000HVHecker/CashFlow/" + connection.selectConstContent("arbeitnehmerkonstanten", "anpersonalnummer", id) + " " + connection.selectConstContent("arbeitnehmerkonstanten", "annachname", id) + "/";
        try {

            PdfWriter.getInstance(document, new FileOutputStream(docfile + docname));
            document.open();

            // Schriftarten festlegen
            BaseFont baseFont = BaseFont.createFont("C:/Windows/Fonts/calibri.ttf", BaseFont.WINANSI, BaseFont.EMBEDDED);
            Font headlinefat = new Font(baseFont, 14, Font.BOLD, BaseColor.BLACK);
            Font headlinenormal = new Font(baseFont, 14, Font.NORMAL, BaseColor.BLACK);
            Font fat = new Font(baseFont, 10, Font.BOLD, BaseColor.BLACK);
            Font anfont = new Font(baseFont, 12, Font.BOLD, BaseColor.BLACK);
            Font normal = new Font(baseFont, 10, Font.NORMAL, BaseColor.BLACK);
            Font small = new Font(baseFont, 8, Font.NORMAL, BaseColor.BLACK);

            // Erste Zeile
            PdfPTable table = new PdfPTable(5);
            table.setWidthPercentage(100);
            table.setWidths(new float[]{10, 2, 4, 1, 3});
            PdfPCell cell1 = new PdfPCell(new Phrase("Lohnabrechnung der Brutto/ Netto-Bezüge", headlinefat));
            cell1.setBorder(Rectangle.NO_BORDER);
            cell1.setVerticalAlignment(Element.ALIGN_MIDDLE);
            table.addCell(cell1);
            PdfPCell spacer1 = new PdfPCell();
            spacer1.setBorder(Rectangle.NO_BORDER);
            table.addCell(spacer1);
            PdfPCell cell2 = new PdfPCell(new Phrase("", headlinenormal));
            cell2.setBorder(Rectangle.NO_BORDER);
            cell2.setVerticalAlignment(Element.ALIGN_LEFT);
            table.addCell(cell2);
            PdfPCell spacer2 = new PdfPCell();
            spacer2.setBorder(Rectangle.NO_BORDER);
            table.addCell(spacer2);
            PdfPCell cell3 = new PdfPCell(new Phrase(datum, normal));
            cell3.setBorder(Rectangle.NO_BORDER);
            cell3.setHorizontalAlignment(Element.ALIGN_RIGHT);
            cell3.setVerticalAlignment(Element.ALIGN_MIDDLE);
            table.addCell(cell3);

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

            //Zweite Zeile
            PdfPTable table2 = new PdfPTable(1);
            table2.setWidthPercentage(100);
            String zeitraum = monat + " " + jahr;
            table2.addCell(createCell(zeitraum, headlinenormal, Element.ALIGN_LEFT, 0, 5, 0, false));

            //Atrribute1
            PdfPTable attributeTable = new PdfPTable(4);
            attributeTable.setWidthPercentage(100);
            attributeTable.setWidths(new float[]{1, 1, 1, 1});
            PdfPCell cell4 = createCell("Personal-Nr.", fat, Element.ALIGN_LEFT, 10, 3, 3, true);
            BaseColor color1 = new BaseColor(211, 211, 211, 225);
            cell4.setBackgroundColor(color1);
            attributeTable.addCell(cell4);
            //attributeTable.addCell(createCell("Personal-Nr.", fat, Element.ALIGN_LEFT, 10, 3, 3,true));
            PdfPCell cell5 = createCell("Geburtsdatum", fat, Element.ALIGN_LEFT, 10, 3, 3, true);
            cell5.setBackgroundColor(color1);
            attributeTable.addCell(cell5);
            //attributeTable.addCell(createCell("Geburtsdatum", fat, Element.ALIGN_LEFT, 10, 3, 3,true));
            PdfPCell cell6 = createCell("SV-Nummer", fat, Element.ALIGN_LEFT, 10, 3, 3, true);
            cell6.setBackgroundColor(color1);
            attributeTable.addCell(cell6);
            //attributeTable.addCell(createCell("SV-Nummer", fat, Element.ALIGN_LEFT, 10, 3, 3,true));
            PdfPCell cell7 = createCell("Krankenkasse", fat, Element.ALIGN_LEFT, 10, 3, 3, false);
            cell7.setBackgroundColor(color1);
            attributeTable.addCell(cell7);
            //attributeTable.addCell(createCell("Krankenkasse", fat, Element.ALIGN_LEFT, 10, 3, 3,false));
            String persnr = connection.selectConstContent("arbeitnehmerkonstanten", "anpersonalnummer", id);
            attributeTable.addCell(createCell(persnr, normal, Element.ALIGN_LEFT, 10, 3, 3, true));
            String gbdatum = connection.selectConstContent("arbeitnehmerkonstanten", "angeburtsdatum", id);
            attributeTable.addCell(createCell(gbdatum, normal, Element.ALIGN_LEFT, 10, 3, 3, true));
            String svnr = connection.selectConstContent("arbeitnehmerkonstanten", "ansvnummer", id);
            attributeTable.addCell(createCell(svnr, normal, Element.ALIGN_LEFT, 10, 3, 3, true));
            attributeTable.addCell(createCell("BUN Die Bundesknappschaft", normal, Element.ALIGN_LEFT, 10, 3, 3, false));

            //Atrribute2
            PdfPTable attributeTable2 = new PdfPTable(4);
            attributeTable2.setWidthPercentage(100);
            attributeTable2.setWidths(new float[]{1, 1, 1, 1});
            PdfPCell cell8 = createCell("PGRS", fat, Element.ALIGN_LEFT, 10, 3, 3, true);
            cell8.setBackgroundColor(color1);
            attributeTable2.addCell(cell8);
            PdfPCell cell9 = createCell("BGRS", fat, Element.ALIGN_LEFT, 10, 3, 3, true);
            cell9.setBackgroundColor(color1);
            attributeTable2.addCell(cell9);
            PdfPCell cell10 = createCell("Steuer-ID", fat, Element.ALIGN_LEFT, 10, 3, 3, true);
            cell10.setBackgroundColor(color1);
            attributeTable2.addCell(cell10);
            PdfPCell cell11 = createCell("Eintritt", fat, Element.ALIGN_LEFT, 10, 3, 3, false);
            cell11.setBackgroundColor(color1);
            attributeTable2.addCell(cell11);
            String pgrs = connection.selectConstContent("arbeitnehmerkonstanten", "anpersonengruppe", id);
            attributeTable2.addCell(createCell(pgrs, normal, Element.ALIGN_LEFT, 10, 3, 3, true));
            String bgrs = connection.selectConstContent("arbeitnehmerkonstanten", "anbgrschluessel", id);
            attributeTable2.addCell(createCell(bgrs, normal, Element.ALIGN_LEFT, 10, 3, 3, true));
            String stid = connection.selectConstContent("arbeitnehmerkonstanten", "ansteuerid", id);
            attributeTable2.addCell(createCell(stid, normal, Element.ALIGN_LEFT, 10, 3, 3, true));
            String eintritt = connection.selectConstContent("arbeitnehmerkonstanten", "anbeschaeftigungsbeginn", id);
            attributeTable2.addCell(createCell(eintritt, normal, Element.ALIGN_LEFT, 10, 3, 3, false));

            //Arbeitgeber Anschrift
            PdfPTable arbeitgeberTable = new PdfPTable(1);
            arbeitgeberTable.setWidthPercentage(100);
            arbeitgeberTable.setWidths(new float[]{1});
            String ag1 = connection.selectConstContent("arbeitnehmerkonstanten", "agname", id).concat(" ").concat(connection.selectConstContent("arbeitnehmerkonstanten", "agname2", id));
            arbeitgeberTable.addCell(createCell(ag1, normal, Element.ALIGN_LEFT, 0, 3, 3, false));
            String ag2 = connection.selectConstContent("arbeitnehmerkonstanten", "agstraße", id).concat(" ").concat(connection.selectConstContent("arbeitnehmerkonstanten", "aghausnummer", id).concat(", ").concat(connection.selectConstContent("arbeitnehmerkonstanten", "agpostleitzahl", id).concat(" ").concat(connection.selectConstContent("arbeitnehmerkonstanten", "agort", id))));
            arbeitgeberTable.addCell(createCell(ag2, normal, Element.ALIGN_LEFT, 0, 3, 3, false));

            //Arbeitgeber Anschrift
            PdfPTable arbeitnehmerTable = new PdfPTable(1);
            arbeitnehmerTable.setWidthPercentage(100);
            arbeitnehmerTable.setWidths(new float[]{1});
            String an1 = connection.selectConstContent("arbeitnehmerkonstanten", "anvorname", id).concat(" ").concat(connection.selectConstContent("arbeitnehmerkonstanten", "annachname", id));
            arbeitnehmerTable.addCell(createCell(an1, anfont, Element.ALIGN_LEFT, 0, 3, 3, false));
            String an2 = connection.selectConstContent("arbeitnehmerkonstanten", "anstraße", id).concat(" ").concat(connection.selectConstContent("arbeitnehmerkonstanten", "anhausnummer", id));
            arbeitnehmerTable.addCell(createCell(an2, anfont, Element.ALIGN_LEFT, 0, 3, 3, false));
            String an3 = connection.selectConstContent("arbeitnehmerkonstanten", "anpostleitzahl", id).concat(" ").concat(connection.selectConstContent("arbeitnehmerkonstanten", "anort", id));
            arbeitnehmerTable.addCell(createCell(an3, anfont, Element.ALIGN_LEFT, 0, 3, 3, false));

            //BruttoAußenTabelle
            PdfPTable bruttoaußenTable = new PdfPTable(3);
            bruttoaußenTable.setWidthPercentage(100);
            bruttoaußenTable.setWidths(new float[]{2, 5, 3});
            bruttoaußenTable.addCell(createCell("Brutto-Bezüge", fat, Element.ALIGN_LEFT, 0, 5, 5, false));
            bruttoaußenTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 5, 5, false));
            PdfPCell betrag = createCell("", fat, Element.ALIGN_RIGHT, 40, 5, 5, false);
            bruttoaußenTable.addCell(betrag);

            //BruttoTabelle
            PdfPTable bruttoTable = new PdfPTable(4);
            bruttoTable.setWidthPercentage(100);
            bruttoTable.setWidths(new float[]{1, 1, 1, 1});
            bruttoTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 10, 10, true));
            bruttoTable.addCell(createCell("Bezeichnung", normal, Element.ALIGN_LEFT, 0, 3, 10, false));
            bruttoTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 10, 10, true));
            bruttoTable.addCell(createCell("Betrag", normal, Element.ALIGN_RIGHT, 30, 3, 10, false));
            bruttoTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 10, 10, false));
            String bezeichnung = "Vergütung ".concat(connection.selectConstContent("arbeitnehmerkonstanten", "anberufsbezeichnung", id));
            bruttoTable.addCell(createCell(bezeichnung, normal, Element.ALIGN_LEFT, 0, 10, 10, false));
            bruttoTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 10, 10, true));
            String svergütung = connection.selectmtlContent(id, "verguetungbrutto" , monat, jahr);
            double dvergütung = Double.parseDouble(svergütung);
            String vergütung = dezimalformat.format(dvergütung).concat(" €");
            bruttoTable.addCell(createCell(vergütung, normal, Element.ALIGN_RIGHT, 30, 10, 10, false));
            bruttoTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 5, 10, false));
            bruttoTable.addCell(createCell("Vergütung Regiestunden", normal, Element.ALIGN_LEFT, 0, 10, 10, false));
            bruttoTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 10, 10, true));
            String sregievergütung = connection.selectmtlContent(id, "sonderverguetungbrutto" , monat, jahr);
            double dregievergütung = Double.parseDouble(sregievergütung);
            String regievergütung = dezimalformat.format(dregievergütung).concat(" €");
            bruttoTable.addCell(createCell(regievergütung, normal, Element.ALIGN_RIGHT, 30, 10, 10, false));

            //BruttoAußenTabelle
            PdfPTable gesamtbruttoaußenTable = new PdfPTable(4);
            gesamtbruttoaußenTable.setWidthPercentage(100);
            gesamtbruttoaußenTable.setWidths(new float[]{1, 1, 1, 1});
            gesamtbruttoaußenTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 5, 0, false));
            gesamtbruttoaußenTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 5, 0, false));
            gesamtbruttoaußenTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 5, 0, true));
            gesamtbruttoaußenTable.addCell(createCell("Gesamt-Brutto", fat, Element.ALIGN_RIGHT, 30, 3, 0, false));
            gesamtbruttoaußenTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 5, 5, false));
            gesamtbruttoaußenTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 5, 5, false));
            gesamtbruttoaußenTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 5, 5, true));
            String sgesamtbetrag = connection.selectmtlContent(id, "gesamtbetragbrutto" , monat, jahr);
            double dgesamtbetrag = Double.parseDouble(sgesamtbetrag);
            String gesamtbetrag = dezimalformat.format(dgesamtbetrag).concat(" €");
            gesamtbruttoaußenTable.addCell(createCell(gesamtbetrag, normal, Element.ALIGN_RIGHT, 30, 10, 10, false));
            gesamtbruttoaußenTable.addCell(createCell("Sozialversicherung", fat, Element.ALIGN_LEFT, 0, 5, 5, false));
            gesamtbruttoaußenTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 5, 5, false));
            gesamtbruttoaußenTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 5, 5, true));
            gesamtbruttoaußenTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 5, 5, false));

            //BruttoTabelle
            PdfPTable kvrvtable = new PdfPTable(4);
            kvrvtable.setWidthPercentage(100);
            kvrvtable.setWidths(new float[]{1, 1, 1, 1});
            kvrvtable.addCell(createCell("", normal, Element.ALIGN_LEFT, 0, 3, 10, true));
            kvrvtable.addCell(createCell("KV-Brutto", normal, Element.ALIGN_LEFT, 0, 3, 10, true));
            kvrvtable.addCell(createCell("RV-Brutto", normal, Element.ALIGN_LEFT, 0, 5, 10, true));
            kvrvtable.addCell(createCell("SV-rechtliche Abzüge", normal, Element.ALIGN_RIGHT, 30, 3, 0, false));
            kvrvtable.addCell(createCell("", normal, Element.ALIGN_LEFT, 0, 10, 10, false));
            kvrvtable.addCell(createCell(gesamtbetrag, normal, Element.ALIGN_LEFT, 0, 10, 10, false));
            kvrvtable.addCell(createCell(gesamtbetrag, normal, Element.ALIGN_LEFT, 0, 10, 10, true));
            kvrvtable.addCell(createCell("0,00 €", normal, Element.ALIGN_RIGHT, 30, 10, 10, false));

            //NettoverdienstTabelle
            PdfPTable nettoTable = new PdfPTable(4);
            nettoTable.setWidthPercentage(100);
            nettoTable.setWidths(new float[]{1, 1, 1, 1});
            nettoTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 10, 0, false));
            nettoTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 10, 0, false));
            nettoTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 10, 0, true));
            nettoTable.addCell(createCell("Netto-Verdienst", fat, Element.ALIGN_RIGHT, 30, 3, 0, false));
            nettoTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 5, 5, false));
            nettoTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 10, 0, false));
            nettoTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 5, 5, true));
            nettoTable.addCell(createCell(gesamtbetrag, normal, Element.ALIGN_RIGHT, 30, 10, 10, false));

            //AuszahlungsbetragTabelle
            PdfPTable auszahlungsbetragTable = new PdfPTable(4);
            auszahlungsbetragTable.setWidthPercentage(100);
            auszahlungsbetragTable.setWidths(new float[]{1, 1, 1, 1});
            auszahlungsbetragTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 10, 0, false));
            auszahlungsbetragTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 10, 0, false));
            auszahlungsbetragTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 10, 0, true));
            auszahlungsbetragTable.addCell(createCell("Auszahlungsbetrag", fat, Element.ALIGN_RIGHT, 30, 3, 0, false));
            auszahlungsbetragTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 10, 0, false));
            auszahlungsbetragTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 10, 0, false));
            auszahlungsbetragTable.addCell(createCell("", fat, Element.ALIGN_LEFT, 0, 10, 0, true));
            auszahlungsbetragTable.addCell(createCell(gesamtbetrag, normal, Element.ALIGN_RIGHT, 30, 10, 10, false));

            //KleingedrucktesTabelle
            PdfPTable kleingedrucktesTable = new PdfPTable(2);
            kleingedrucktesTable.setWidthPercentage(100);
            kleingedrucktesTable.setWidths(new float[]{1, 1});
            kleingedrucktesTable.addCell(createCell("", small, Element.ALIGN_LEFT, 0, 5, 5, false));
            PdfPCell imageCell = new PdfPCell();
            imageCell.setBorder(Rectangle.NO_BORDER);
            Image img = Image.getInstance("C:/Users/Anwender/IdeaProjects/demo3/src/main/resources/Logo1-removebg.png");
            img.scaleToFit(30, 30);
            img.setAlignment(Element.ALIGN_RIGHT);
            imageCell.addElement(img);
            imageCell.setPaddingRight(6);
            kleingedrucktesTable.addCell(imageCell);
            kleingedrucktesTable.addCell(createCell("Dieses Dokument wurde von CashFlow generiert.", small, Element.ALIGN_LEFT, 0, 5, 5, false));
            kleingedrucktesTable.addCell(createCell("CashFlow", fat, Element.ALIGN_RIGHT, 0, 5, 5, false));

            //Inhalte hinzufügen
            document.add(table);
            document.add(table2);
            document.add(new Paragraph(" "));
            document.add(longline);
            document.add(attributeTable);
            document.add(new Paragraph(" "));
            document.add(attributeTable2);
            document.add(longline);
            document.add(new Paragraph(" "));
            document.add(new Paragraph(" "));
            document.add(arbeitgeberTable);
            document.add(new Paragraph(" "));
            document.add(arbeitnehmerTable);
            document.add(new Paragraph(" "));
            document.add(bruttoaußenTable);
            document.add(longline);
            document.add(bruttoTable);
            document.add(longline);
            document.add(gesamtbruttoaußenTable);
            document.add(longline);
            document.add(kvrvtable);
            document.add(longline);
            document.add(nettoTable);
            document.add(shortline);
            document.add(auszahlungsbetragTable);
            document.add(shortline);
            document.add(new Paragraph(" "));
            document.add(new Paragraph(" "));
            document.add(new Paragraph(" "));
            document.add(new Paragraph(" "));
            document.add(kleingedrucktesTable);
            document.close();
            CONFIRMPopup confirmPopup = new CONFIRMPopup();
            confirmPopup.display("CONFIRM99.fxml");
        } catch (DocumentException | IOException e) {
            e.printStackTrace();
            try {
                Main.changeScene("/LAZA2.fxml");
                CONFIRMPopup confirmPopup = new CONFIRMPopup();
                confirmPopup.display("NOTCONFIRM99.fxml");
            }
            catch (Exception e1) {
                e1.printStackTrace();
            }

        }
    }

    private PdfPCell createCell(String content, Font font, int alignment, float paddingLeftRight, float paddingTop, float paddingBottom, boolean rightborder) {
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


