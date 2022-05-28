/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

// CPP
pub const TARGET_CPP_CMP: &str = r#"
#include <bits/stdc++.h>
using namespace std;
int main() {
    int n; cin >> n;
    vector<int> arr(n);
    for(auto &a: arr) cin >> a;
    sort(arr.begin(), arr.end());
    cout << n << endl;
    for(auto &a: arr) cout << a << " ";
    cout << endl;
    return 0;
}"#;

pub const CORRECT_CPP_CMP: &str = r#"
#include <bits/stdc++.h>
using namespace std;
int main() {
    int n; cin >> n;
    vector<int> arr(n);
    for(auto &a: arr) cin >> a;
    for (int i = 0; i < n-1; i++) {
        for (int j = 0; j < n-i-1; j++) {
            if (arr[j] > arr[j+1]) {
                swap(arr[j], arr[j+1]);
            }
        }
    }
    cout << n << endl;
    for(auto &a: arr) cout << a << " ";
    cout << endl;
    return 0;
}"#;

pub const GEN_CPP_CMP: &str = r#"
#include <bits/stdc++.h>
using namespace std;
template <typename T>
T random(const T from, const T to) {
    static random_device rdev;
    static default_random_engine re(rdev());
    using dist_type = typename conditional<
        is_floating_point<T>::value,
        uniform_real_distribution<T>,
        uniform_int_distribution<T>
    >::type;
    dist_type uni(from, to);
    return static_cast<T>(uni(re));
}
int main() {
    int n = random<int>(1e2, 2e2);
    cout << n << endl;
    for(int i=0;i<n;++i) cout << random<int>(1, 1e9) << " ";
    cout << endl;
    return 0;
}"#;

// PYTHON
pub const TARGET_PY_CMP: &str = r#"
n = int(input())
A = list(map(int, input().split()))
A.sort()
print(n)
print(*A)"#;

pub const CORRECT_PY_CMP: &str = r#"
n = int(input())
A = list(map(int, input().split()))

for i in range(n-1):
    for j in range(n-i-1):
        if A[j] > A[j+1]:
            A[j], A[j+1] = A[j+1], A[j]

print(n)
print(*A)"#;

pub const GEN_PY_CMP: &str = r#"
from random import uniform
n = int(uniform(int(1e2), int(2e2)))
print(n)
A = [int(uniform(1, int(1e9))) for _ in range(n)]
print(*A)"#;

// C
pub const TARGET_C_CMP: &str = r#"
#include <stdio.h>
#include <stdlib.h>
int a[100001];
int cmp (const void * a, const void * b) { return ( *(int*)a - *(int*)b ); }
int main() {
    int n;
    scanf("%d", &n);
    for(int i = 0; i < n; ++i) scanf("%d", &a[i]);
    qsort(a, n, sizeof(int), cmp);
    printf("%d\n", n);
    for(int i = 0; i < n; ++i) {
        if(i > 0) printf(" ");
        printf("%d", a[i]);
    }
    return 0;
}"#;

pub const CORRECT_C_CMP: &str = r#"
#include <stdio.h>
int a[100001];
void swap(int *xp, int *yp) { int temp = *xp; *xp = *yp; *yp = temp; }
int main() {
    int n;
    scanf("%d", &n);
    for(int i = 0; i < n; ++i) scanf("%d", &a[i]);
    for (int i = 0; i < n-1; i++)
        for (int j = 0; j < n-i-1; j++)
            if (a[j] > a[j+1])
                swap(&a[j], &a[j+1]);
    printf("%d\n", n);
    for(int i = 0; i < n; ++i) {
        if(i > 0) printf(" ");
        printf("%d", a[i]);
    }
    return 0;
}
"#;

pub const GEN_C_CMP: &str = r#"
#include <stdio.h>
int main(int argc, char *argv[]) {
    int seed;
    sscanf(argv[1], "%d", &seed);
    srand(seed); 
    int r = rand();
    int n = 100 + rand() % (200 - 100 + 1 );
    printf("%d\n", n);

    for(int i = 0; i < n; i++) {
        if(i > 0) printf(" ");
        printf("%d",  1 + rand() % ( 1000000000 - 1 + 1 ));
    }
    fflush(stdout);
    return 0;
}"#;

// RUST
pub const TARGET_RUST_CMP: &str = r#"
#![allow(warnings, unused)]
use proconio::input;
use std::io::{BufWriter, StdoutLock, Write};

fn main() {
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    input! {
        n: usize,
        mut a: [i64; n]
    }
    a.sort();
    writeln!(out, "{}", n).ok();
    for i in 0..n {
        if i > 0 {
            write!(out, " ").ok();
        }
        write!(out, "{}", n).ok();
    }
    out.flush();
}"#;

pub const CORRECT_RUST_CMP: &str = r#"
#![allow(warnings, unused)]
use proconio::input;
use std::io::{BufWriter, StdoutLock, Write};

fn main() {
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    input! {
        n: usize,
        mut a: [i64; n]
    }
    for i in 0..n-1 {
        for j in 0..n-i-1 {
            if a[j] > a[j+1] {
                a.swap(j, j+1);
            }
        }
    }
    writeln!(out, "{}", n).ok();
    for i in 0..n {
        if i > 0 {
            write!(out, " ").ok();
        }
        write!(out, "{}", n).ok();
    }
    out.flush();
}"#;

pub const GEN_RUST_CMP: &str = r#"
#![allow(warnings, unused)]
use std::io::{BufWriter, StdoutLock, Write};
use rand::Rng;
fn main() {
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut rng = rand::thread_rng();
    let N: i32 = 1_000;
    let Ai: i32 = 100_000;
    let n: i32 = rng.gen_range(1..=N);
    writeln!(out, "{}", n).ok();
    for i in 0..n {
        if i > 0 {
            write!(out, " ").ok();
        }
        write!(out, "{}", rng.gen_range(-Ai..=Ai)).ok();
    }
    out.flush();
}"#;

// GO
pub const TARGET_GO_CMP: &str = r#"
package main
import (
	"bufio"
	"fmt"
	"os"
    "sort"
)
var r = bufio.NewReader(os.Stdin)
var w = bufio.NewWriter(os.Stdout)
func main() {
    var n int
	fmt.Fscan(r, &n)
	values := make([]int, n)
	for i:=0;i<n;i++ {
		fmt.Fscan(r, &values[i])
	}
    sort.Ints(values)
    fmt.Fprintln(w, n)
    for i:=0; i<n ;i++ {
        if i > 0 {
            fmt.Fprint(w, " ")
        }
        fmt.Fprint(w, values[i])
    }
    
	defer w.Flush()
}"#;

pub const CORRECT_GO_CMP: &str = r#"
package main
import (
	"bufio"
	"fmt"
	"os"
)
var r = bufio.NewReader(os.Stdin)
var w = bufio.NewWriter(os.Stdout)
func main() {
    var n int
	fmt.Fscan(r, &n)
	values := make([]int, n)
	for i:=0;i<n;i++ {
		fmt.Fscan(r, &values[i])
	}
    for i:=0; i< len(values)-1; i++ {
        for j:=0; j < len(values)-i-1; j++ {
           if (values[j] > values[j+1]) {
              values[j], values[j+1] = values[j+1], values[j]
           }
        }
     }
    fmt.Fprintln(w, n)
    for i:=0; i<n ;i++ {
        if i > 0 {
            fmt.Fprint(w, " ")
        }
        fmt.Fprint(w, values[i])
    }
    
	defer w.Flush()
}"#;

pub const GEN_GO_CMP: &str = r#"
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
    N := 100
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
}"#;

// JAVA
pub const TARGET_JAVA_CMP: &str = r#"
import java.util.*;
import java.io.PrintWriter;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        PrintWriter out = new PrintWriter(System.out);
        int n = sc.nextInt();
        int[] values = new int[n];
        for(int i = 0; i < n; ++i) {
            values[i] = sc.nextInt();
        }
        Arrays.sort(values);
        out.println(n);
        for(int i = 0; i < n; ++i) {
            if(i > 0)
                out.print(" ");
            out.print(values[i]);
        }
        out.flush();
    }
}"#;

pub const CORRECT_JAVA_CMP: &str = r#"
import java.util.*;
import java.io.PrintWriter;
public class Correct {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        PrintWriter out = new PrintWriter(System.out);
        int n = sc.nextInt();
        int[] values = new int[n];
        for(int i = 0; i < n; ++i) {
            values[i] = sc.nextInt();
        }
        for (int i = 0; i < n-1; i++) {
            for (int j = 0; j < n-i-1; j++) {
                if (values[j] > values[j+1]) {
                    Correct.swap(values, j, j+1);
                }
            }
        }
        out.println(n);
        for(int i = 0; i < n; ++i) {
            if(i > 0)
                out.print(" ");
            out.print(values[i]);
        }
        out.flush();
    }
    public static void swap(int[] a, int i, int j) {
        int t = a[i];
        a[i] = a[j];
        a[j] = t;
    }
}"#;

pub const GEN_JAVA_CMP: &str = r#"
import java.util.Random;
import java.io.PrintWriter;

public class Gen {
    public static void main(String[] args) {
        Random rand = new Random();
        PrintWriter out = new PrintWriter(System.out);
        
        final int N  = 1000;
        final int mn = -100000;
        final int mx = 100000;

        int n = rand.nextInt(N);

        out.println(n);

        for(int i = 0; i < n; i++) {
            if(i > 0)
                out.print(" ");
            int ai = (int)Math.floor(Math.random()*(mx-mn+1)+mn);
            out.print(ai);
        }
        out.flush();
    }
}"#;

// KOTLIN
pub const _TARGET_KOTLIN_CMP: &str = r#"
fun main() {
    val n = readLine()!!.toInt()
    val a = readLine()!!.split(" ").map { it.toInt() }
    val out = StringBuilder()
    var sorted = a.sorted()
    out.appendln(n)
    for(i in 0..n-1) {
        if(i > 0) {
            out.append(" ")
        }
        out.append(sorted[i])
    }
    println(out)
}
"#;

pub const _CORRECT_KOTLIN_CMP: &str = r#"
fun main() {
    val n = readLine()!!.toInt()
    val a = readLine()!!.split(" ").map { it.toInt() }.toMutableList()
    val out = StringBuilder()
    for (i in 0..n-1) {
        for (j in 0..n-i-2) {
            if (a[j] > a[j+1]) {
                val tmp = a[j]
                a[j] = a[j + 1]
                a[j + 1] = tmp
            }
        }
    }
    out.appendln(n)
    for(i in 0..n-1) {
        if(i > 0) {
            out.append(" ")
        }
        out.append(a[i])
    }
    println(out)
}
"#;

pub const _GEN_KOTLIN_CMP: &str = r#"
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
}"#;
