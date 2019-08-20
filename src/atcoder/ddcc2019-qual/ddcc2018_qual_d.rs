// https://atcoder.jp/contests/ddcc2019-qual/tasks/ddcc2018_qual_d
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

    ($iter:expr, [ $t:tt ; $len:expr ; $offset:expr ; $default:expr ]) => {
        (0..$len+$offset).map(|i|
            if i >= $offset {
                read_value!($iter, $t)
            } else {
                $default
            }
        ).collect::<Vec<_>>()
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

const MAX: i64 = 1e12 as i64;


/// Returns (g, x, y) where:
/// - ax + by = g
/// - g = gcd(a, b)
fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a.abs(), a.signum(), 0)
    } else {
        let (g, x, y) = ext_gcd(b, a % b);
        (g, y, x - a / b * y)
    }
}

/// Returns (r, m) such that x ≡ r (mod m) holds, where:
/// - for each i = 1 to b.len(), x ≡ b_i (mod w_i)
/// If there is no such pair, returns (0, -1) instead.
fn chinise_remainder_theorem(b: &Vec<i64>, w: &Vec<i64>) -> (i64, i64) {
    let mut r = 0;
    let mut m = 1;
    for i in 0..b.len() {
        let (g, p, q) = ext_gcd(m, w[i]);
        if (b[i] - r) % g != 0 {
            return (0, -1);
        }
        let step = (b[i] - r) / g * p % (w[i] / g);
        r += m * step;
        m *= w[i] / g;
    }
    ((r % m + m) % m, m)
}

fn wsum(num: i64, n: i64) -> i64 {
    let mut w = 0;
    let mut num = num;
    while num >= 1 {
        w += num % n;
        num /= n;
    }
    w
}

fn is_valid(num: i64, a: &Vec<i64>) -> bool {
    for i in 2..a.len() {
        if wsum(num, i as i64) != a[i] {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        a: [i64; 29; 2; 0]
    };

    let m: Vec<i64> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let mut b = vec![];
    for &z in &m {
        b.push(a[z as usize + 1]);
    }
    let (mut r, m) = chinise_remainder_theorem(&b, &m);
    while m != -1 && r <= MAX {
        if is_valid(r, &a) {
            println!("{}", r);
            return;
        }
        r += m;
    }
    println!("invalid");
}
