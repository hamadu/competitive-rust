// https://atcoder.jp/contests/agc034/tasks/agc034_c
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

type Test = (i64, i64, i64);

fn isok(time: i64, x: i64, tests: &Vec<Test>) -> bool {
    let n = tests.len();
    let ocnt = (time / x) as usize;
    let md = time % x;
    if ocnt >= n {
        return true;
    }

    let mut ok = false;

    let mut ls = 0;
    for i in 0..n {
        ls += tests[i].0 * tests[i].1;
    }
    let mut rs0 = 0;
    let mut rs1 = 0;
    for i in 0..ocnt+1 {
        let w = tests[i].0 * tests[i].1 + (x - tests[i].0) * tests[i].2;
        if i < ocnt {
            rs0 += w;
        }
        rs1 += w;
    }

    for i in 0..n {
        let mut rs = rs0;
        if ocnt > i {
            rs = rs1;
            rs -= tests[i].0 * tests[i].1 + (x - tests[i].0) * tests[i].2;
        }
        if md < tests[i].0 {
            rs += md * tests[i].1;
        } else {
            rs += tests[i].0 * tests[i].1 + (md - tests[i].0) * tests[i].2;
        }
        ok |= rs >= ls;
    }
    ok
}

fn main() {
    input! {
        n: usize, x: i64,
        tests: [(i64, i64, i64); n]
    };

    let mut tests = tests;
    tests.sort_by(|t0, t1| {
        let a = (x - t0.0) * t0.2 + t0.0 * t0.1;
        let b = (x - t1.0) * t1.2 + t1.0 * t1.1;
        return b.partial_cmp(&a).unwrap();
    });


    let mut ok = n as i64 * x;
    let mut ng = -1;
    while ok - ng > 1 {
        let med = (ok + ng) / 2;
        if isok(med, x, &tests) {
            ok = med;
        } else {
            ng = med;
        }
    }
    println!("{}", ok);
}
