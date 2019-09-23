// https://atcoder.jp/contests/yahoo-procon2019-qual/tasks/yahoo_procon2019_qual_e
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

    ($iter:expr, [ next / $t:tt ]) => {
        {
            let len = read_value!($iter, usize);
            (0..len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
        }
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


const MOD: i64 = 998244353;

fn rank(mut a: Vec<Vec<i32>>) -> usize {
    let n = a.len();
    let m = a[0].len();
    let mut rank = 0;
    for j in 0..m {
        'lp: for i in rank..n {
            if a[i][j] == 1 {
                if i != rank {
                    for k in 0..m {
                        let tmp = a[rank][k];
                        a[rank][k] = a[i][k];
                        a[i][k] = tmp;
                    }
                }
                for d in 0..n {
                    if d != rank && a[d][j] == 1 {
                        for k in 0..m {
                            a[d][k] ^= a[rank][k];
                        }
                    }
                }
                rank += 1;
                break 'lp;
            }
        }
    }
    // debug!(a, rank);
    rank
}

fn main() {
    input! {
        n: usize, m: usize,
        a: [[i32; m]; n]
    };
    let mut p2 = vec![0; n+m];
    p2[0] = 1;
    for i in 1..n+m {
        p2[i] = p2[i-1] * 2 % MOD;
    }


    let rank = rank(a);
    let ans = p2[m-1] * p2[n-rank] % MOD * (p2[rank] + MOD - 1) % MOD;

    println!("{}", ans);
}
