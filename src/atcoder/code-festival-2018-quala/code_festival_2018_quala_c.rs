// https://atcoder.jp/contests/code-festival-2018-quala/tasks/code_festival_2018_quala_c
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

const MOD: i64 = 1000000007;

fn main() {
    input! {
        n: usize, k: usize,
        a: [i64; n],
    };

    let mut variance = vec![0; n];
    for i in 0..n {
        let mut aa = a[i];
        while aa >= 1 {
            variance[i] += 1;
            aa /= 2;
        }
    }

    // println!("{:?}", variance);

    let mut dp = dvec!(0; n+1, 2, 3100);
    dp[0][0][0] = 1;
    for i in 0..n {
        for f in 0..2 {
            for w in 0..3100 {
                let base = dp[i][f][w];
                if base == 0 {
                    continue;
                }

                for z in 0..variance[i]+1 {
                    let mut tf = f;
                    if z == variance[i] {
                        tf = 1;
                    }
                    dp[i+1][tf][w+z] += base;
                    dp[i+1][tf][w+z] %= MOD;
                }
            }
        }
    }

    let mut sum = 0;
    for i in 0..min(3100, k) {
        sum += dp[n][1][i];
    }
    if k < 3100 {
        sum += dp[n][0][k] + dp[n][1][k];
    }
    println!("{}", sum % MOD);
}
