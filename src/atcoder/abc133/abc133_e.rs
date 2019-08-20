// https://atcoder.jp/contests/abc133/tasks/abc133_e
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

const MOD: i64 = 1e9 as i64 + 7;

fn dfs(graph: &Vec<Vec<usize>>, now: usize, par: usize, k: i64) -> i64 {
    let mut ways = 1;
    let mut colors = 0;
    if par == graph.len() {
        colors = k-1;
    } else {
        colors = k-2;
    }
    for &to in &graph[now] {
        if to == par {
            continue;
        }
        ways *= colors;
        ways %= MOD;
        ways *= dfs(graph, to, now, k);
        ways %= MOD;
        if colors == 0 {
            break;
        }
        colors -= 1;
    }
    ways
}

fn main() {
    input! {
        n: usize, k: i64,
        edges: [(usize1, usize1); n-1]
    };

    let mut graph = vec![vec![]; n];
    for e in edges {
        graph[e.0].push(e.1);
        graph[e.1].push(e.0);
    }

    println!("{}", k * dfs(&graph, 0, n, k) % MOD);
}
