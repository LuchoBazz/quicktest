/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

// CODES
pub const TARGET_CPP_OUTPUT_CMD: &str = r#"
#include <bits/stdc++.h>
using namespace std;
int n;
vector<int> A;
int main() {
    cin >> n;
    A.resize(n);
    for(auto &ref: A) cin >> ref;
    sort(A.begin(), A.end());
    cout << n << endl;
    for(auto &a: A) cout << a << " ";
    cout << endl;
    return 0;
}
"#;

pub const TARGET_PY_OUTPUT_CMD: &str = r#"
n = int(input())
A = list(map(int, input().split()))
A.sort()
print(n)
print(*A)
"#;

pub const TARGET_C_OUTPUT_CMD: &str = r#"
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

pub const TARGET_RUST_OUTPUT_CMD: &str = r#"
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

pub const TARGET_GO_OUTPUT_CMD: &str = r#"
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

pub const TARGET_JAVA_OUTPUT_CMD: &str = r#"
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
