object MatMul {
  {
    val pwd = new java.io.File( "." ).getAbsolutePath
    System.load(s"${pwd}/../rust-matmul/target/debug/libmatmul.dylib")
  }

  @native def matmul(m1: Matrix, m2: Matrix, mout: Matrix): Unit;

  def time[R](block: => R): R = {
    val t0 = System.nanoTime()
    val result = block
    val t1 = System.nanoTime()
    println("Elapsed time: " + (t1 - t0) + "ns")
    result
  }

  val N = 4
  type Matrix = Array[Array[Int]]

  def init(m: Matrix): Matrix = {
    val r = scala.util.Random
    for(i <- 0 until N; j <- 0 until N) m(i)(j) = r.nextInt(N)
    m
  }

  def matmul2(m1: Matrix, m2: Matrix, mout: Matrix): Unit = {
    for(i <- 0 until N; j <- 0 until N; k <- 0 until N) mout(i)(j) += m1(i)(k) * m2(k)(j)
  }

  def matprint(m: Matrix): Unit = {
    for(i <- 0 until N) {
      for(j <- 0 until N) {
        print(s"${m(i)(j)} ")
      }
      println("")
    }
  }

  def main(args: Array[String]): Unit = {
    val m1 = init(Array.ofDim[Int](N, N))
    val m2 = init(Array.ofDim[Int](N, N))
    var mout = Array.ofDim[Int](N, N)
    var mout2 = Array.ofDim[Int](N, N)

    matprint(m1); println()
    matprint(m2); println()

    time {matmul2(m1, m2, mout2)}
    matprint(mout2); println()

    time {matmul(m1, m2, mout)}
    matprint(mout); println()
  }
}
