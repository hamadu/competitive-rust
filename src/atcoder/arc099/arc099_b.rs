// https://atcoder.jp/contests/arc099/tasks/arc099_b
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

fn s(n: i64) -> i64 {
    let mut n = n;
    let mut sum = 0;
    while n >= 1 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

fn rate(n: i64) -> f64 {
    (n as f64) / (s(n) as f64)
}

fn gen(f: i64, sum: i64, keta: i64) -> i64 {
    if sum > keta * 9 {
        return -1;
    }
    let mut ret = 0;
    let mut left = sum;
    let mut p10 = 1;
    for i in 0..keta-1 {
        ret += p10 * min(9, left);
        left -= min(9, left);
        p10 *= 10;
    }
    ret + p10 * f
}

/// determines if n/S(n) > m/S(m) holds or not.
fn larger(n: i64, m: i64) -> bool {
    n * s(m) > m * s(n)
}

fn gengen() {
    let max = 1000000i64;
    for i in 1..max {
        let mut ok = true;
        for j in i..max {
            if larger(i, j) {
                ok = false;
                break;
            }
        }
        if ok {
            println!("{}", i);
        }
    }
}

fn main() {
    input! {
        k: i64
    };

    // gengen();

    let mut gset = HashSet::new();
    for i in 1..16 {
        for j in 0..16*9 {
            for f in 1..11 {
                let cd = gen(f, j, i);
                if cd >= 1 && cd <= 1e15 as i64 {
                    gset.insert(cd);
                }
            }
        }
    }

    let mut g = vec![];
    for gi in gset {
        g.push(gi);
    }
    g.sort();
    // debug!(g.len());

    let mut left = k;
    for i in 0..g.len() {
        let mut ok = true;
        for j in i..g.len() {
            if larger(g[i], g[j]) {
                ok = false;
                break;
            }
        }
        if ok && left >= 1 {
            left -= 1;
            println!("{}", g[i]);
        }
    }
    // debug!(k-left);
}
