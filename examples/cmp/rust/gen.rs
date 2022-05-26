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
}
 