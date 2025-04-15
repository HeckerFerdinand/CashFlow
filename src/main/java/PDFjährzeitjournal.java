import com.itextpdf.text.*;
import com.itextpdf.text.pdf.*;
import com.itextpdf.text.pdf.draw.LineSeparator;
import java.io.FileOutputStream;
import java.io.IOException;
import java.text.DecimalFormat;


public class PDFjährzeitjournal {


    public void print(String id, String datum, String jahr) {
        Document document = new Document();
        document.setPageSize(PageSize.A4.rotate());
        document.setMargins(30, 30, 5, 5);
        SQLConnectionBase connection = new SQLConnectionBase();
        DecimalFormat dezimalformat = new DecimalFormat("#,##0.00");
        String docname = jahr + "-12-31 " + connection.selectConstContent("arbeitnehmerkonstanten", "anpersonalnummer", id) + " " + connection.selectConstContent("arbeitnehmerkonstanten", "annachname", id) + " Zeitjournal.pdf";
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
            String title = "Zeitjournal ".concat(jahr);
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
            //"Arbeitszeit"
            bruttotable.addCell(createCell("Arbeitszeit", normal, Element.ALIGN_LEFT, 10, 10, 3, false));
            String svb1 = connection.selectmtlContent(id, "arbeitszeit", "Januar", jahr);
            double dvb1 = Double.parseDouble(svb1);
            String vb1 = dezimalformat.format(dvb1).concat(" h");
            bruttotable.addCell(createCell(vb1, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String svb2 = connection.selectmtlContent(id, "arbeitszeit", "Februar", jahr);
            double dvb2 = Double.parseDouble(svb2);
            String vb2 = dezimalformat.format(dvb2).concat(" h");
            bruttotable.addCell(createCell(vb2, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String svb3 = connection.selectmtlContent(id, "arbeitszeit", "März", jahr);
            double dvb3 = Double.parseDouble(svb3);
            String vb3 = dezimalformat.format(dvb3).concat(" h");
            bruttotable.addCell(createCell(vb3, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String svb4 = connection.selectmtlContent(id, "arbeitszeit", "April", jahr);
            double dvb4 = Double.parseDouble(svb4);
            String vb4 = dezimalformat.format(dvb4).concat(" h");
            bruttotable.addCell(createCell(vb4, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String svb5 = connection.selectmtlContent(id, "arbeitszeit", "Mai", jahr);
            double dvb5 = Double.parseDouble(svb5);
            String vb5 = dezimalformat.format(dvb5).concat(" h");
            bruttotable.addCell(createCell(vb5, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String svb6 = connection.selectmtlContent(id, "arbeitszeit", "Juni", jahr);
            double dvb6 = Double.parseDouble(svb6);
            String vb6 = dezimalformat.format(dvb6).concat(" h");
            bruttotable.addCell(createCell(vb6, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String svb7 = connection.selectmtlContent(id, "arbeitszeit", "Juli", jahr);
            double dvb7 = Double.parseDouble(svb7);
            String vb7 = dezimalformat.format(dvb7).concat(" h");
            bruttotable.addCell(createCell(vb7, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String svb8 = connection.selectmtlContent(id, "arbeitszeit", "August", jahr);
            double dvb8 = Double.parseDouble(svb8);
            String vb8 = dezimalformat.format(dvb8).concat(" h");
            bruttotable.addCell(createCell(vb8, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String svb9 = connection.selectmtlContent(id, "arbeitszeit", "September", jahr);
            double dvb9 = Double.parseDouble(svb9);
            String vb9 = dezimalformat.format(dvb9).concat(" h");
            bruttotable.addCell(createCell(vb9, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String svb10 = connection.selectmtlContent(id, "arbeitszeit", "Oktober", jahr);
            double dvb10 = Double.parseDouble(svb10);
            String vb10 = dezimalformat.format(dvb10).concat(" h");
            bruttotable.addCell(createCell(vb10, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String svb11 = connection.selectmtlContent(id, "arbeitszeit", "November", jahr);
            double dvb11 = Double.parseDouble(svb11);
            String vb11 = dezimalformat.format(dvb11).concat(" h");
            bruttotable.addCell(createCell(vb11, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            String svb12 = connection.selectmtlContent(id, "arbeitszeit", "Dezember", jahr);
            double dvb12 = Double.parseDouble(svb12);
            String vb12 = dezimalformat.format(dvb12).concat(" h");
            bruttotable.addCell(createCell(vb12, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            double doublevbgesamt = dvb1 + dvb2 + dvb3 + dvb4 + dvb5 + dvb6 + dvb7 + dvb8 + dvb9 + dvb10 + dvb11 + dvb12;
            String vbgesamt = dezimalformat.format(doublevbgesamt).concat(" h");
            bruttotable.addCell(createCell(vbgesamt, normal, Element.ALIGN_RIGHT, 10, 10, 3, false));
            //"Arbeitszeit Regie"
            bruttotable.addCell(createCell("Arbeitszeit Regie", normal, Element.ALIGN_LEFT, 10, 5, 3, false));
            String ssvb1 = connection.selectmtlContent(id, "arbeitszeitregie", "Januar", jahr);
            double dsvb1 = Double.parseDouble(ssvb1);
            String vsb1 = dezimalformat.format(dsvb1).concat(" h");
            bruttotable.addCell(createCell(vsb1, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String ssvb2 = connection.selectmtlContent(id, "arbeitszeitregie", "Februar", jahr);
            double dsvb2 = Double.parseDouble(ssvb2);
            String vsb2 = dezimalformat.format(dsvb2).concat(" h");
            bruttotable.addCell(createCell(vsb2, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String ssvb3 = connection.selectmtlContent(id, "arbeitszeitregie", "März", jahr);
            double dsvb3 = Double.parseDouble(ssvb3);
            String vsb3 = dezimalformat.format(dsvb3).concat(" h");
            bruttotable.addCell(createCell(vsb3, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String ssvb4 = connection.selectmtlContent(id, "arbeitszeitregie", "April", jahr);
            double dsvb4 = Double.parseDouble(ssvb4);
            String vsb4 = dezimalformat.format(dsvb4).concat(" h");
            bruttotable.addCell(createCell(vsb4, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String ssvb5 = connection.selectmtlContent(id, "arbeitszeitregie", "Mai", jahr);
            double dsvb5 = Double.parseDouble(ssvb5);
            String vsb5 = dezimalformat.format(dsvb5).concat(" h");
            bruttotable.addCell(createCell(vsb5, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String ssvb6 = connection.selectmtlContent(id, "arbeitszeitregie", "Juni", jahr);
            double dsvb6 = Double.parseDouble(ssvb6);
            String vsb6 = dezimalformat.format(dsvb6).concat(" h");
            bruttotable.addCell(createCell(vsb6, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String ssvb7 = connection.selectmtlContent(id, "arbeitszeitregie", "Juli", jahr);
            double dsvb7 = Double.parseDouble(ssvb7);
            String vsb7 = dezimalformat.format(dsvb7).concat(" h");
            bruttotable.addCell(createCell(vsb7, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String ssvb8 = connection.selectmtlContent(id, "arbeitszeitregie", "August", jahr);
            double dsvb8 = Double.parseDouble(ssvb8);
            String vsb8 = dezimalformat.format(dsvb8).concat(" h");
            bruttotable.addCell(createCell(vsb8, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String ssvb9 = connection.selectmtlContent(id, "arbeitszeitregie", "September", jahr);
            double dsvb9 = Double.parseDouble(ssvb9);
            String vsb9 = dezimalformat.format(dsvb9).concat(" h");
            bruttotable.addCell(createCell(vsb9, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String ssvb10 = connection.selectmtlContent(id, "arbeitszeitregie", "Oktober", jahr);
            double dsvb10 = Double.parseDouble(ssvb10);
            String vsb10 = dezimalformat.format(dsvb10).concat(" h");
            bruttotable.addCell(createCell(vsb10, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String ssvb11 = connection.selectmtlContent(id, "arbeitszeitregie", "November", jahr);
            double dsvb11 = Double.parseDouble(ssvb11);
            String vsb11 = dezimalformat.format(dsvb11).concat(" h");
            bruttotable.addCell(createCell(vsb11, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            String ssvb12 = connection.selectmtlContent(id, "arbeitszeitregie", "Dezember", jahr);
            double dsvb12 = Double.parseDouble(ssvb12);
            String vsb12 = dezimalformat.format(dsvb12).concat(" h");
            bruttotable.addCell(createCell(vsb12, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double doublesvbgesamt = dsvb1 + dsvb2 + dsvb3 + dsvb4 + dsvb5 + dsvb6 + dsvb7 + dsvb8 + dvb9 + dsvb10 + dsvb11 + dsvb12;
            String vsbgesamt = dezimalformat.format(doublesvbgesamt).concat(" h");
            bruttotable.addCell(createCell(vsbgesamt, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            //"Gesamtarbeitszeit"
            bruttotable.addCell(createCell("Gesamtarbeitszeit", fat, Element.ALIGN_LEFT, 10, 5, 3, false));
            double dgb1 = dvb1 + dsvb1;
            String gb1 = dezimalformat.format(dgb1).concat(" h");
            bruttotable.addCell(createCell(gb1, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double dgb2 = dvb2 + dsvb2;
            String gb2 = dezimalformat.format(dgb2).concat(" h");
            bruttotable.addCell(createCell(gb2, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double dgb3 = dvb3 + dsvb3;
            String gb3 = dezimalformat.format(dgb3).concat(" h");
            bruttotable.addCell(createCell(gb3, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double dgb4 = dvb4 + dsvb4;
            String gb4 = dezimalformat.format(dgb4).concat(" h");
            bruttotable.addCell(createCell(gb4, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double dgb5 = dvb5 + dsvb5;
            String gb5 = dezimalformat.format(dgb5).concat(" h");
            bruttotable.addCell(createCell(gb5, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double dgb6 = dvb6 + dsvb6;
            String gb6 = dezimalformat.format(dgb6).concat(" h");
            bruttotable.addCell(createCell(gb6, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double dgb7 = dvb7 + dsvb7;
            String gb7 = dezimalformat.format(dgb7).concat(" h");
            bruttotable.addCell(createCell(gb7, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double dgb8 = dvb8 + dsvb8;
            String gb8 = dezimalformat.format(dgb8).concat(" h");
            bruttotable.addCell(createCell(gb8, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double dgb9 = dvb9 + dsvb9;
            String gb9 = dezimalformat.format(dgb9).concat(" h");
            bruttotable.addCell(createCell(gb9, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double dgb10 = dvb10 + dsvb10;
            String gb10 = dezimalformat.format(dgb10).concat(" h");
            bruttotable.addCell(createCell(gb10, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double dgb11 = dvb11 + dsvb11;
            String gb11 = dezimalformat.format(dgb11).concat(" h");
            bruttotable.addCell(createCell(gb11, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double dgb12 = dvb12 + dsvb12;
            String gb12 = dezimalformat.format(dgb12).concat(" h");
            bruttotable.addCell(createCell(gb12, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double dbggesamt = dgb1 + dgb2 + dgb3 + dgb4 + dgb5 + dgb6 + dgb7 + dgb8 + dgb9 + dgb10 + dgb11 + dgb12;
            String sbggesamt = dezimalformat.format(dbggesamt).concat(" h");
            bruttotable.addCell(createCell(sbggesamt, fat, Element.ALIGN_RIGHT, 10, 5, 3, false));


            PdfPTable beitragTable = new PdfPTable(14);
            beitragTable.setWidthPercentage(100);
            beitragTable.setWidths(new float[]{4,2,2,2,2,2,2,2,2,2,2,2,2,3});
            //Urlaubstage
            beitragTable.addCell(createCell("Urlaubstage", normal, Element.ALIGN_LEFT, 10, 5, 3, false));
            double skv1 = connection.selectmtldoubleContent(id, "urlaub", "Januar", jahr);
            String kv1 = dezimalformat.format(skv1).concat(" d");
            beitragTable.addCell(createCell(kv1, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double dkv2 = connection.selectmtldoubleContent(id, "urlaub", "Februar", jahr);
            String kv2 = dezimalformat.format(dkv2).concat(" d");
            beitragTable.addCell(createCell(kv2, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double skv3 = connection.selectmtldoubleContent(id, "urlaub", "März", jahr);
            String kv3 = dezimalformat.format(skv3).concat(" d");
            beitragTable.addCell(createCell(kv3, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double skv4 = connection.selectmtldoubleContent(id, "urlaub", "April", jahr);
            String kv4 = dezimalformat.format(skv4).concat(" d");
            beitragTable.addCell(createCell(kv4, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double skv5 = connection.selectmtldoubleContent(id, "urlaub", "Mai", jahr);
            String kv5 = dezimalformat.format(skv5).concat(" d");
            beitragTable.addCell(createCell(kv5, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double skv6 = connection.selectmtldoubleContent(id, "urlaub", "Juni", jahr);
            String kv6 = dezimalformat.format(skv6).concat(" d");
            beitragTable.addCell(createCell(kv6, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double skv7 = connection.selectmtldoubleContent(id, "urlaub", "Juli", jahr);
            String kv7 = dezimalformat.format(skv7).concat(" d");
            beitragTable.addCell(createCell(kv7, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double skv8 = connection.selectmtldoubleContent(id, "urlaub", "August", jahr);
            String kv8 = dezimalformat.format(skv8).concat(" d");
            beitragTable.addCell(createCell(kv8, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double skv9 = connection.selectmtldoubleContent(id, "urlaub", "September", jahr);
            String kv9 = dezimalformat.format(skv9).concat(" d");
            beitragTable.addCell(createCell(kv9, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double skv10 = connection.selectmtldoubleContent(id, "urlaub", "Oktober", jahr);
            String kv10 = dezimalformat.format(skv10).concat(" d");
            beitragTable.addCell(createCell(kv10, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double skv11 = connection.selectmtldoubleContent(id, "urlaub", "November", jahr);
            String kv11 = dezimalformat.format(skv11).concat(" d");
            beitragTable.addCell(createCell(kv11, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double skv12 = connection.selectmtldoubleContent(id, "urlaub", "Dezember", jahr);
            double dkv12;
            dkv12 = skv12;
            String kv12 = dezimalformat.format(dkv12).concat(" d");
            beitragTable.addCell(createCell(kv12, normal, Element.ALIGN_RIGHT, 10, 5, 3, false));
            double dkvgesamt = skv1 + dkv2 + skv3 + skv4 + skv5 + skv6 + skv7 + skv8 + skv9 + skv10 + skv11 + dkv12;
            String skvgesamt = dezimalformat.format(dkvgesamt).concat(" d");
            beitragTable.addCell(createCell(skvgesamt, normal, Element.ALIGN_RIGHT, 10, 5, 5, false));
            //Krankheitstage
            beitragTable.addCell(createCell("Krankheitstage", normal, Element.ALIGN_LEFT, 10, 5, 10, false));
            double drv1 = connection.selectmtldoubleContent(id, "krank", "Januar", jahr);
            String rv1 = dezimalformat.format(drv1).concat(" d");
            beitragTable.addCell(createCell(rv1, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            double drv2 = connection.selectmtldoubleContent(id, "krank", "Februar", jahr);
            String rv2 = dezimalformat.format(drv2).concat(" d");
            beitragTable.addCell(createCell(rv2, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            double drv3 = connection.selectmtldoubleContent(id, "krank", "März", jahr);
            String rv3 = dezimalformat.format(drv3).concat(" d");
            beitragTable.addCell(createCell(rv3, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            double drv4 = connection.selectmtldoubleContent(id, "krank", "April", jahr);
            String rv4 = dezimalformat.format(drv4).concat(" d");
            beitragTable.addCell(createCell(rv4, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            double drv5 = connection.selectmtldoubleContent(id, "krank", "Mai", jahr);
            String rv5 = dezimalformat.format(drv5).concat(" d");
            beitragTable.addCell(createCell(rv5, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            double drv6 = connection.selectmtldoubleContent(id, "krank", "Juni", jahr);
            String rv6 = dezimalformat.format(drv6).concat(" d");
            beitragTable.addCell(createCell(rv6, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            double srv7 = connection.selectmtldoubleContent(id, "krank", "Juli", jahr);
            String rv7 = dezimalformat.format(srv7).concat(" d");
            beitragTable.addCell(createCell(rv7, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            double srv8 = connection.selectmtldoubleContent(id, "krank", "August", jahr);
            String rv8 = dezimalformat.format(srv8).concat(" d");
            beitragTable.addCell(createCell(rv8, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            double srv9 = connection.selectmtldoubleContent(id, "krank", "September", jahr);
            String rv9 = dezimalformat.format(srv9).concat(" d");
            beitragTable.addCell(createCell(rv9, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            double drv10 = connection.selectmtldoubleContent(id, "krank", "Oktober", jahr);
            String rv10 = dezimalformat.format(drv10).concat(" d");
            beitragTable.addCell(createCell(rv10, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            double drv11 = connection.selectmtldoubleContent(id, "krank", "November", jahr);
            String rv11 = dezimalformat.format(drv11).concat(" d");
            beitragTable.addCell(createCell(rv11, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            double drv12 = connection.selectmtldoubleContent(id, "krank", "Dezember", jahr);
            String rv12 = dezimalformat.format(drv12).concat(" d");
            beitragTable.addCell(createCell(rv12, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));
            double drvgesamt = drv1 + drv2 + drv3 + drv4 + drv5 + drv6 + srv7 + srv8 + srv9 + drv10 + drv11 + drv12;
            String srvgesamt = dezimalformat.format(drvgesamt).concat(" d");
            beitragTable.addCell(createCell(srvgesamt, normal, Element.ALIGN_RIGHT, 10, 5, 10, false));

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
            document.add(beitragTable);
            document.add(longline);
            document.add(new Paragraph(" "));
            document.add(new Paragraph(" "));
            document.add(new Paragraph(" "));
            document.add(new Paragraph(" "));
            document.add(new Paragraph(" "));
            document.add(new Paragraph(" "));
            document.add(new Paragraph(" "));
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



