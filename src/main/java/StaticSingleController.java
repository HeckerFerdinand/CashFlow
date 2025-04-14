public class StaticSingleController {
    private static ANBE3Controller anbe3Controller;
    private static ANBE4Controller anbe4Controller;
    private static ANBE5Controller anbe5Controller;

    public static ANBE3Controller getANBE3Controller() {
        if (anbe3Controller == null) {
            anbe3Controller = new ANBE3Controller();
        }
        return anbe3Controller;
    }

    public static ANBE4Controller getANBE4Controller() {
        if (anbe4Controller == null) {
            anbe4Controller = new ANBE4Controller();
        }
        return anbe4Controller;
    }

    public static ANBE5Controller getANBE5Controller() {
        if (anbe5Controller == null) {
            anbe5Controller = new ANBE5Controller();
        }
        return anbe5Controller;
    }
}

