// https://atcoder.jp/contests/abc134/tasks/abc134_f
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

const MOD: u64 = 1000000007;

fn powmod(a: u64, p: u64, m: u64) -> u64 {
    let mut ret = 1u64;
    let mut aa = a;
    let mut pp = p;
    while pp >= 1 {
        if pp & 1 == 1 {
            ret *= aa;
            ret %= m;
        }
        aa = aa * aa % m;
        pp >>= 1;
    }
    ret
}

fn inv(a: u64, m: u64) -> u64 {
    powmod(a, m-2, m)
}


fn main() {
    input! {
        n: usize, k: usize,
    };

    let m = 2500 + 1;
    let mut dp = dvec!(0; n+1, n+1, 2*m+1);
    dp[0][0][m] = 1;
    for i in 0..n {
        let x = i+1;
        for f in 0..i+1 {
            let both = (i-f) as u64;
            let left = (n-2*f-both as usize) as u64;
            for l in 0..2*m+1 {
                let base = dp[i][f][l];
                if base == 0 {
                    continue;
                }

                // -, -
                if left >= 1 {
                    if left >= 2 {
                        dp[i+1][f+1][l-x*2] += base * left % MOD * (left - 1) % MOD;
                        dp[i+1][f+1][l-x*2] %= MOD;
                    }
                    dp[i+1][f][l] += base * left % MOD;
                    dp[i+1][f][l] %= MOD;
                }

                // -, +
                if left >= 1 && f >= 1 {
                    dp[i+1][f][l] += base * left % MOD * (f as u64) % MOD * 2 % MOD;
                    dp[i+1][f][l] %= MOD;
                }

                // +, +
                if f >= 1 {
                    dp[i+1][f-1][l+x*2] += base * (f * f) as u64 % MOD;
                    dp[i+1][f-1][l+x*2] %= MOD;
                }
            }
        }
    }

    let mut f = 1;
    for i in 1..n+1 {
        f *= i as u64;
        f %= MOD;
    }
    let iv = inv(f, MOD);

    println!("{}", dp[n][0][m+k] * iv % MOD);
}
