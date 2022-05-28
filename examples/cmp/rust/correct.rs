#![allow(warnings, unused)]
use std::cmp::{min, max};
use std::io::{BufWriter, StdoutLock, Write};
use std::collections::{BTreeSet, BTreeMap, HashSet, HashMap, BinaryHeap, VecDeque};

type Set<T> = BTreeSet<T>;
type Map<K, V> = BTreeMap<K, V>;
type Heap<T> = BinaryHeap<T>;
type Deque<T> = VecDeque<T>;

fn main() {

    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc: Scanner = Scanner::new(std::io::stdin().lock());
    let mut io = IO { sc: sc, out: out };
    
    input!(sc=io.sc, n: usize, mut a: [i64; n]);

    let mut best = 0i64;
    for i in 0..n {
        let mut sum = 0i64;
        for j in i..n {
            sum += a[j];
            best = max(best, sum);
        }
    }

    println!("{}", best);
}
 
// reference: https://github.com/uesugi6111/io_pro
#[macro_export]
macro_rules! input{
    (sc=$sc:expr,$($r:tt)*)=>{ input_inner!{$sc,$($r)*} };
    ($($r:tt)*)=>{let mut sc=fast_input::Scanner::new(std::io::stdin().lock());input_inner!{sc,$($r)*}};
}
#[macro_export]
macro_rules! input_inner{
    ($sc:expr)=>{};
    ($sc:expr,)=>{};
    ($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};
    ($sc:expr,mut $var:ident:$t:tt$($r:tt)*)=>{let mut $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};
}
#[macro_export]
macro_rules! read_value{
    ($sc:expr,($($t:tt),*))=>{($(read_value!($sc,$t)),*)};
    ($sc:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()};
    ($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};
    ($sc:expr,Usize1)=>{read_value!($sc,usize)-1};
    ($sc:expr,$t:ty)=>{$sc.next::<$t>()};
}
struct IO<'a> { sc: Scanner, out: BufWriter<StdoutLock<'a>> }
pub struct Scanner {s: Box<str>, input: std::iter::Peekable<std::str::SplitAsciiWhitespace<'static>>,}
impl Scanner {
    pub fn new<R: std::io::Read>(mut reader: R) -> Self {let s = {let mut s = String::new();reader.read_to_string(&mut s).unwrap();s.into_boxed_str()};let mut sc = Scanner {s,input: "".split_ascii_whitespace().peekable(),};use std::mem;let s: &'static str = unsafe { mem::transmute(&*sc.s) };sc.input = s.split_ascii_whitespace().peekable();sc}
    #[inline]
    pub fn next<T: std::str::FromStr>(&mut self) -> T where T::Err: std::fmt::Debug,{self.input.next().unwrap().parse::<T>().expect("Parse Error")}
}
#[macro_export]
macro_rules! debug {
    ($($a:expr),*) => {
        #[cfg(debug_assertions)]
        writeln!(&mut std::io::stderr(), concat!("[DEBUG] ", $(stringify!($a), "={:?} "),*), $($a),*);
    }
}