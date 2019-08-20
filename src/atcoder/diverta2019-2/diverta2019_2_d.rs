// https://atcoder.jp/contests/diverta2019-2/tasks/diverta2019_2_d
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
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

const INF: i64 = -1;
const PAR: usize = 5000;

fn sum(n: usize, x0: usize, y0: usize, z0: usize, x1: i64, y1: i64, z1: i64) -> i64 {
    let mut dp = vec![INF; PAR*2+1];

    let mut best = n as i64;
    dp[0] = 0;
    for i in 0..n+1 {
        let fr = i % PAR;
        let base = dp[fr];
        if base != INF {
            best = max(best, base+(n-i) as i64);
            if i == n {
                break;
            }
            dp[fr+x0] = max(dp[fr+x0], base+x1);
            dp[fr+y0] = max(dp[fr+y0], base+y1);
            dp[fr+z0] = max(dp[fr+z0], base+z1);
        }

        if (i+1) % PAR == 0 {
            for j in 0..PAR {
                dp[j] = dp[PAR+j];
            }
        }
    }
    // println!("{:?}", dp);
    best
}

fn main() {
    input! {
        n: usize,
        x0: usize, y0: usize, z0: usize,
        x1: usize, y1: usize, z1: usize
    };

    let s1 = sum(n, x0, y0, z0, x1 as i64, y1 as i64, z1 as i64);
    let s2 = sum(s1 as usize, x1, y1, z1, x0 as i64, y0 as i64, z0 as i64);

    println!("{}", s2);
}
