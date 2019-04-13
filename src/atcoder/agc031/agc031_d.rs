// https://atcoder.jp/contests/agc031/tasks/agc031_d
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

fn inv(p: &Vec<usize>) -> Vec<usize> {
    let n = p.len();
    let mut q = vec![0; n];
    for i in 0..n {
        q[p[i]] = i;
    }
    return q;
}

fn mul(p: &Vec<usize>, q: &Vec<usize>) -> Vec<usize> {
    let n = p.len();
    let mut r = vec![0; n];
    for i in 0..n {
        r[i] = q[p[i]];
    }
    return r;
}

fn pow(p: &Vec<usize>, k: u64) -> Vec<usize> {
    let n = p.len();
    let mut r = vec![0; n];
    for i in 0..n {
        r[i] = i;
    }
    let mut pp = p.clone();
    let mut k = k;
    while k >= 1 {
        if k & 1 == 1 {
            r = mul(&r, &pp);
        }
        pp = mul(&pp, &pp);
        k >>= 1;
    }
    r
}

fn solve_small(p0: Vec<usize>, q0: Vec<usize>, k: u64) -> Vec<usize> {
    if k == 1 {
        return p0;
    }
    if k == 2 {
        return q0;
    }
    let mut p = p0;
    let mut q = q0;
    for _i in 3..k + 1 {
        let rev = inv(&p);
        let neq = mul(&rev, &q);
        p = q.clone();
        q = neq;
    }
    q
}

fn solve(p0: Vec<usize>, q0: Vec<usize>, k: u64) -> Vec<usize> {
    if k <= 20 {
        return solve_small(p0, q0, k);
    }
    let p1 = inv(&p0);
    let q1 = inv(&q0);

    let prefix = mul(&mul(&mul(&q1, &p0), &q0), &p1);
    let suffix = mul(&mul(&mul(&p0, &q1), &p1), &q0);

    let prcount = (k - 2) / 6;
    let sucount = (k + 1) / 6;
    let center = (k - 2) % 6;

    let p = pow(&prefix, prcount);
    let s = pow(&suffix, sucount);
    let c = match center {
        0 => q0.clone(),
        1 => mul(&p1, &q0),
        2 => mul(&mul(&q1, &p1), &q0),
        3 => q1.clone(),
        4 => mul(&q1, &p0),
        5 => mul(&mul(&q1, &p0), &q0),
        _ => panic!("no"),
    };
    mul(&mul(&p, &c), &s)
}

fn main() {
    input! {
        n: usize, k: u64,
        p: [usize1; n],
        q: [usize1; n]
    };

    let ans = solve(p, q, k);

    for i in 0..n {
        if i >= 1 {
            print!(" ");
        }
        print!("{}", ans[i] + 1);
    }
    println!();
}
