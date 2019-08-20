// https://atcoder.jp/contests/code-festival-2018-qualb/tasks/code_festival_2018_qualb_d
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

fn main() {
    input! {
        n: usize, m: usize, q: f64,
        rates: [(f64, f64); m]
    };

    let mut psum = vec![0f64; m+1];
    for i in 0..m {
        let p = rates[i].1 / q;
        psum[i+1] = psum[i] + p;
    }

    let mut fact_ln = vec![0f64; n+1];
    fact_ln[0] = 1f64.ln();
    for i in 1..n+1 {
        fact_ln[i] = fact_ln[i-1] + (i as f64).ln();
    }

    let mut distributions = dvec!(0.0; n, m+1);
    for i in 0..n {
        let lnum = i as f64;
        let rnum = (n-i) as f64;
        distributions[i][0] = 1f64;
        for j in 1..m {
            let left = psum[j].ln();
            let right = (1f64 - psum[j]).ln();
            distributions[i][j] = (left * lnum + right * rnum + fact_ln[n] - fact_ln[i] - fact_ln[n-i]).exp();
        }
    }
    for i in 0..n-1 {
        for j in 1..m {
            distributions[i+1][j] += distributions[i][j];
        }
    }

    // debug!(distributions);

    let mut ans = 0f64;
    for i in 0..n {
        let mut center = -1f64;
        let mut sum_prob = 0f64;
        for j in 0..m {
            let prob = distributions[i][j] - distributions[i][j+1];
            sum_prob += prob;
            if sum_prob >= 0.5 && center == -1f64 {
                center = rates[j].0;
            }
        }
        for j in 0..m {
            let prob = distributions[i][j] - distributions[i][j+1];
            ans += (center - rates[j].0).abs() * prob;
        }
    }


    println!("{}", ans);
}
