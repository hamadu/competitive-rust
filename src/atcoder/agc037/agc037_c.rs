// https://atcoder.jp/contests/agc037/tasks/agc037_c
//
#![allow(unused_imports)]
use std::io::*;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused_macros)]
macro_rules! dvec {
    ($t:expr ; $len:expr) => {
        vec![$t; $len]
    };

    ($t:expr ; $len:expr, $($rest:expr),*) => {
        vec![dvec!($t; $($rest),*); $len]
    };
}

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

fn solve(mut a: Vec<i64>, mut b: Vec<i64>) -> i64 {
    let n = a.len();
    for i in 0..n {
        if b[i] < a[i] {
            return -1;
        }
    }
    let mut pq = BinaryHeap::new();
    for i in 0..n {
        if a[i] < b[i] {
            pq.push((b[i], i));
        }
    }

    let mut step = 0;
    while let Some((mut v, i)) = pq.pop() {
        let lr = b[(i+1)%n] + b[(i+n-1)%n];
        if lr > v {
            return -1;
        }
        assert!(a[i] < b[i]);
        assert!(b[i] == v);
        let sage = v - a[i];
        let k = sage / lr;
        if k == 0 {
            return -1;
        }
        v -= lr * k;
        b[i] = v;
        if a[i] > b[i] {
            return -1;
        } else if a[i] < b[i] {
            pq.push((b[i], i));
        }
        step += k;
    }

    for i in 0..n {
        if a[i] != b[i] {
            return -1;
        }
    }
    step
}

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n]
    };

    println!("{}", solve(a, b));
}
