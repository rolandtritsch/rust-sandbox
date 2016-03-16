import java.nio.file.Path;
import java.nio.file.Paths;

class Adder {
    static {
	Path p = Paths.get("../rust-adder/target/debug/libadder.dylib");
	System.load(p.toAbsolutePath().toString());
    }

    public static native int add(int v1, int v2);

    public static void main(String... args) {
	if(args.length != 2) {
	    System.out.println("Usage: Adder <x> <y>");
	    System.exit(1);
	}

	int x = Integer.parseInt(args[0]);
	int y = Integer.parseInt(args[1]);

	System.out.println(x + " + " + y + " = " + Adder.add(x, y));
	System.exit(0);
    }
}
