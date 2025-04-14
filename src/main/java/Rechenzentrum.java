public class Rechenzentrum {


    public  double calcgesamtbetragbrutto(double verguetungbrutto, double sonderverguetungbrutto)
    {
        return sonderverguetungbrutto+verguetungbrutto;
    }

    public double calckva(double gesamtbeitragbrutto, double kv)
    {
        double kva = gesamtbeitragbrutto*kv;
        kva = Math.round(kva * 100.0) / 100.0;
        return kva;
    }

    public double calcrva(double gesamtbeitragbrutto, double rv)
    {
        double rva = gesamtbeitragbrutto*rv;
        rva = Math.round(rva * 100.0) / 100.0;
        return rva;
    }

    public double calcu1a(double gesamtbeitragbrutto, double u1)
    {
        double u1a = gesamtbeitragbrutto*u1;
        u1a = Math.round(u1a * 100.0) / 100.0;
        return u1a;
    }

    public double calcu2a(double gesamtbeitragbrutto, double u2)
    {
        double u2a = gesamtbeitragbrutto*u2;
        u2a = Math.round(u2a * 100.0) / 100.0;
        return u2a;
    }

    public double calcinsoa(double gesamtbeitragbrutto, double inso)
    {
        double insoa = gesamtbeitragbrutto*inso;
        insoa = Math.round(insoa * 100.0) / 100.0;
        return insoa;
    }

    public double calcsta(double gesamtbeitragbrutto, double st)
    {
        double sta = gesamtbeitragbrutto*st;
        sta = Math.round(sta * 100.0) / 100.0;
        return sta;
    }

    public double calcgesamtbeitrag(double kva, double rva, double u1a, double u2a, double insoa, double sta)
    {
        double gesamtbeitrag = kva+rva+u1a+u2a+insoa+sta;
        gesamtbeitrag = Math.round(gesamtbeitrag*100.0) / 100.0;
        return gesamtbeitrag;
    }

    public String calcgesamtarbeitszeitzeit(String arbeitszeit, String arbeitszeitregie)
    {
        double gesamtarbeitszeitd = Double.parseDouble(arbeitszeit)+Double.parseDouble(arbeitszeitregie);
        String gesamtarbeitszeit = String.valueOf(gesamtarbeitszeitd);
        return gesamtarbeitszeit;
    }
}
