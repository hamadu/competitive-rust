// https://atcoder.jp/contests/yahoo-procon2019-qual/tasks/yahoo_procon2019_qual_f
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

fn ctou(c: char) -> usize {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!("no"),
    }
}

const MOD: i64 = 998244353;

fn main() {
    input! {
        s: chars
    };

    let n = s.len();
    let mut dp = dvec![0; 2*n+1, 2*n+1];

    dp[0][0] = 1;
    let mut blue = 0;
    let mut red = 0;
    for i in 0..2*n {
        if i < n {
            blue += ctou(s[i]);
            red += 2-ctou(s[i]);
        }
        for j in 0..2*n+1 {
            let base = dp[i][j];
            if base == 0 {
                continue;
            }

            if j < blue {
                dp[i+1][j+1] += base;
                dp[i+1][j+1] %= MOD;
            }
            if (i-j) < red {
                dp[i+1][j] += base;
                dp[i+1][j] %= MOD;
            }
        }
    }

    println!("{}", dp[2*n][blue]);
}
