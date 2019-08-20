// https://atcoder.jp/contests/agc035/tasks/agc035_c
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

#[allow(unused_macros)]
macro_rules! dvec {
    ($t:expr ; $len:expr) => {
        vec![$t; $len]
    };

    ($t:expr ; $len:expr, $($rest:expr),*) => {
        vec![dvec!($t; $($rest),*); $len]
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

fn main() {
    input! {
        n: usize,
    };

    let mut nn = n;
    while nn > 1 && nn % 2 == 0 {
        nn /= 2;
    }
    if nn == 1 {
        println!("No");
        return;
    }


    let mut edges = vec![];
    edges.push((1, 2));
    edges.push((2, 3));
    edges.push((3, n+1));
    edges.push((n+1, n+2));
    edges.push((n+2, n+3));

    let mut k = 5;
    while k <= n {
        let l = k-1;
        edges.push((1, l));
        edges.push((l, k));
        edges.push((1, n+k));
        edges.push((n+k, n+l));
        k += 2;
    }
    if n % 2 == 0 {
        let mut w = 2;
        loop {
            if n & w == w {
                let c = n - w;
                edges.push((w, n));
                edges.push((n+c+1, 2*n));
                break;
            }
            w <<= 1;
        }
    }

    println!("Yes");
    for &e in &edges {
        println!("{} {}", e.0, e.1);
    }
}
