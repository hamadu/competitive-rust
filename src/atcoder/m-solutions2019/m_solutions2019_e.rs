// https://atcoder.jp/contests/m-solutions2019/tasks/m_solutions2019_e
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


const MOD: usize = 1e6 as usize + 3;

fn powmod(a: usize, p: usize, m: usize) -> usize {
    let mut ret = 1usize;
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

fn inv(a: usize, m: usize) -> usize {
    powmod(a, m-2, m)
}

fn go(from: usize, n: usize, tbl: &Vec<usize>) -> usize {
    assert!(from >= 1);
    if from + n - 1 >= MOD {
        return 0;
    }
    tbl[from+n-1] * inv(tbl[from-1], MOD) % MOD
}

fn main() {
    input! {
        q: usize,
        queries: [(usize, usize, usize); q]
    };

    let mut tbl = vec![0; MOD];
    tbl[0] = 0;
    tbl[1] = 1;
    for i in 2..MOD {
        tbl[i] = tbl[i-1] * i % MOD;
    }

    for &(x, d, n) in &queries {
        if x == 0 {
            println!("{}", 0);
        } else if d == 0 {
            println!("{}", powmod(x, n, MOD) % MOD);
        } else {
            let from = x * inv(d, MOD) % MOD;
            let pw = powmod(d, n, MOD);
            println!("{}", go(from, n, &tbl) * pw % MOD);
        }
    }
}
