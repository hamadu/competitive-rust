// https://atcoder.jp/contests/abc138/tasks/abc138_d
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

fn dfs(now: usize, par: usize, g: &Vec<Vec<usize>>, w: &Vec<i64>, score: &mut Vec<i64>, mut add: i64) {
    add += w[now];
    score[now] += add;
    for &to in &g[now] {
        if to == par {
            continue;
        }
        dfs(to, now, g, w, score, add);
    }
}

fn main() {
    input! {
        n: usize, q: usize,
        edges: [(usize1, usize1); n-1],
        queries: [(usize1, i64); q]
    };

    let mut g = vec![vec![]; n];
    for (u, v) in edges {
        g[u].push(v);
        g[v].push(u);
    }

    let mut weight = vec![0; n];
    for (i, v) in queries {
        weight[i] += v;
    }

    let mut score = vec![0; n];
    dfs(0, n, &g, &weight, &mut score, 0);

    println!("{}", score.into_iter().map(|s| s.to_string()).collect::<Vec<_>>().join(" "));
}
