package main

import (
	"bufio"
	"fmt"
	"os"
)

var r = bufio.NewReader(os.Stdin)
var w = bufio.NewWriter(os.Stdout)

func solveOne() {
    var n int
	fmt.Fscan(r, &n)
	values := make([]int, n)
	for i:=0;i<n;i++ {
		fmt.Fscan(r, &values[i])
	}
    var sum, best int
    sum = 0
    best = 0
    for i:=0;i+1<n;i++ {
        sum = max(values[i], sum + values[i]);
        best = max(best, sum);
    }
    fmt.Fprintln(w, best)
}

func min(a, b int) int {if a < b {return a} else {return b}}
func max(a, b int) int {if a > b {return a} else {return b}}

func main() {
	defer w.Flush()
	solveOne()
}




