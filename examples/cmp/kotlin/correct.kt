fun main() {
    // Maximum Subarray Problem

    val n = readLine()!!.toInt()
    val a = readLine()!!.split(" ").map { it.toInt() }

    var best = 0
    
    // n-2 was added intensively to generate an error
    for (i in 0..n-1) {
        var sum = 0
        for (j in i..n-1) {
            sum += a[j];
            best = best.coerceAtLeast(sum);
        }
    }
    println(best)
}