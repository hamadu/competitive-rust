// https://atcoder.jp/contests/agc031/tasks/agc031_b
//
#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use std::fmt::*;
use std::io::*;
use std::str::*;

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

const MOD: i64 = 1000000007;

fn main() {
    input! {
        n: usize,
        c: [usize; n]
    };

    let mut a = vec![];
    let mut last = 2000000;
    for col in c {
        if last != col {
            a.push(col);
        }
        last = col;
    }

    let n = a.len();
    let mut sum = vec![0; 200010];
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for i in 0..n {
        dp[i + 1] = (dp[i] + sum[a[i]]) % MOD;
        sum[a[i]] = (sum[a[i]] + dp[i]) % MOD;
    }

    println!("{}", dp[n]);
}
