// https://atcoder.jp/contests/ddcc2019-qual/tasks/ddcc2018_qual_c
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

const MOD: i64 = 1e9 as i64 + 7;

fn main() {
    input! {
        n: usize
    };

    let mut c = dvec!(0; 30, 30);
    for i in 0..c.len() {
        c[i][0] = 1;
        c[i][i] = 1;
        for j in 1..i {
            c[i][j] = (c[i-1][j-1] + c[i-1][j]) % MOD;
        }
    }

    let mut ways = vec![0i64; n+1];
    ways[1] = 1;
    for p in 2..n+1 {
        let mut total = 0;
        let mut wo = 1;
        for s in 0..10 {
            total += wo * c[10][s];
            total %= MOD;
            wo *= (p-1) as i64;
            wo %= MOD;
        }
        ways[p] = total;
    }

    let mut ans = 0;
    for i in 1..n+1 {
        for j in 1..n+1 {
            if i * j > n {
                break;
            }
            ans += ways[i] * ways[j] % MOD;
            ans %= MOD;
        }
    }

    println!("{}", ans);
}
