#![allow(warnings, unused)]
use proconio::input;
use std::cmp::{min, max};

fn main() {
    
    input! {
        n: usize,
        mut a: [i64; n]
    }

    let mut best = 0i64;
    
    // n-1 was added intensively to generate an error
    for i in 0..n {
        let mut sum = 0i64;
        for j in 0..n {
            sum += a[j];
            best = max(best, sum);
        }
    }

    println!("{}", best);
}
 