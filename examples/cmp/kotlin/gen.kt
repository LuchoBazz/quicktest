import kotlin.random.Random
fun random(random: Random, from: Int, to: Int): Int {
    return (random.nextInt(from, to)).toInt()
}
fun main(args: Array<String>) {
    var seed = args[1].toInt();
    var rand = Random(seed)
    val out = StringBuilder()
    var N  = 1000
    var Ai = 100000 
    var n = random(rand, 1, N)
    out.appendLine(n)
    for (i in 0..n-1) {
        if(i > 0) {
            out.append(" ")
        }
        out.append(random(rand, -Ai, Ai))
    }
    println(out)
}