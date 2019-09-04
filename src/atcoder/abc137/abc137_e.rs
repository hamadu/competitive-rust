// https://atcoder.jp/contests/abc137/tasks/abc137_e
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

type Edge = (usize, i64);

const INF: i64 = 1e12 as i64;

fn iterate(dp: &mut Vec<i64>, graph: &Vec<Vec<Edge>>, up: bool) {
    let n = graph.len();
    for i in 0..n {
        if dp[i] == -INF {
            continue;
        }
        for &(to, cost) in &graph[i] {
            if dp[to] < dp[i] + cost {
                dp[to] = if up { INF } else { dp[i]+cost }
            }
        }
    }
}

fn solve(graph: Vec<Vec<Edge>>) -> i64 {
    let n = graph.len();
    let mut dp = vec![-INF; n];
    dp[0] = 0;

    for _ in 0..2*n {
        iterate(&mut dp, &graph, false);
    }

    for _ in 0..2*n {
        iterate(&mut dp, &graph, true);
    }
    if dp[n-1] >= INF {
        return -1;
    }
    max(0, dp[n-1])
}

fn main() {
    input! {
        n: usize, m: usize, p: i64,
        edges: [(usize1, usize1, i64); m]
    };
    let mut graph = vec![vec![]; n];
    for (u, v, c) in edges {
        graph[u].push((v, c-p));
    }
    println!("{}", solve(graph));
}
