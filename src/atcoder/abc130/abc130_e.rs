// https://atcoder.jp/contests/abc130/tasks/abc130_e
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
macro_rules! ifv {
    ($t:expr, $a:expr, $b: expr) => {
        if $t { $a } else { $b }
    }
}

#[allow(unused_macros)]
macro_rules! fill {
    ($t:expr, $v:expr) => {
        for i in 0..$t.len() {
            $t[i] = $v;
        }
    };
}

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

const MOD: i64 = 1e9 as i64 + 7;

fn main() {
    input! {
        n: usize, m: usize,
        s: [i32; n],
        t: [i32; m]
    };

    let mut dp = dvec!(0; n+1, m+1);
    let mut dp2 = dvec!(0; n+1, m+1);
    dp[0][0] = 0;
    dp2[0][0] = 0;
    for i in 0..n+1 {
        for j in 0..m+1 {
            if i >= 1 && j >= 1 {
                if s[i-1] == t[j-1] {
                    dp[i][j] = dp2[i-1][j-1] + 1;
                    dp[i][j] %= MOD;
                }
                dp2[i][j] = dp2[i-1][j] + dp2[i][j-1] + MOD - dp2[i-1][j-1];
                dp2[i][j] += dp[i][j];
                dp2[i][j] %= MOD;
            }
        }
    }

    let mut hoge = 1;
    for i in 0..n+1 {
        for j in 0..m+1 {
            hoge += dp[i][j];
            hoge %= MOD;
        }
    }
    println!("{}", hoge);
}
