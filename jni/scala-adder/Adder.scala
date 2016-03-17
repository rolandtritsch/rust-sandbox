object Adder {
  {
    val pwd = new java.io.File( "." ).getAbsolutePath
    System.load(s"${pwd}/../rust-adder/target/debug/libadder.dylib")
  }

  @native def add(x: Int, y: Int): Int

  def main(args: Array[String]): Unit = {
    if (args.size != 2) {
      println("Usage: Adder <x> <y>")
      System.exit(1)
    }

    val x = args(0).toInt
    val y = args(1).toInt

    println(s"${x} + ${y} = ${add(x, y)}")
  }
}
