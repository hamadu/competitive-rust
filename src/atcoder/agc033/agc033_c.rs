// https://atcoder.jp/contests/agc033/tasks/agc033_c
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

type V = usize;
type Graph = Vec<Vec<V>>;

fn diameter(g: Graph) -> usize {
    let (best_leaf, _) = dfs(&g, 0, g.len());
    let (_, diameter) = dfs(&g, best_leaf, g.len());
    diameter
}

fn dfs(g: &Graph, v: V, p: V) -> (V, usize) {
    let mut best_depth = 0;
    let mut best_leaf = v;
    for &t in &g[v] {
        if t == p {
            continue;
        }
        let (l, d) = dfs(g, t, v);
        if best_depth < d+1 {
            best_depth = d+1;
            best_leaf = l;
        }
    }
    (best_leaf, best_depth)
}

fn main() {
    input! {
        n: usize,
        edges: [(usize1, usize1); n-1]
    };

    let mut g = vec![vec![]; n];
    for e in edges {
        g[e.0].push(e.1);
        g[e.1].push(e.0);
    }

    let d = diameter(g);
    if (d+2) % 3 == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}

