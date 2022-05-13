#![allow(warnings, unused)]
use proconio::input;
use std::cmp::{min, max};

fn main() {
    
    input! {
        n: usize,
        mut a: [i64; n]
    }

    let mut best = 0i64;
    let mut sum = 0i64;

    // n-1 was added intensively to generate an error
    for i in 0..n-1 {
        sum = max(a[i], sum + a[i]);
        best = max(best, sum);
    }

    println!("{}", best);
}
 