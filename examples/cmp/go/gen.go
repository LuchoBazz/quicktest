package main

import (
    "bufio"
    "fmt"
    "math/rand"
    "strconv"
    "os"
)

var r = bufio.NewReader(os.Stdin)
var w = bufio.NewWriter(os.Stdout)

func main() {
    args := os.Args
    seed, _ := strconv.Atoi(args[1])
    
    rand.Seed(int64(seed))
    
    N := 500
    mn := -100000
    mx :=  100000
    n := rand.Intn(N - 1) + 1
    fmt.Fprintln(w, n);
    
    for i := 0 ; i < n; i++ {
        if i > 0 {
            fmt.Fprint(w, " ");    
        }
        fmt.Fprint(w, rand.Intn(mx - mn) + mn);    
    }
    
    defer w.Flush()
}
