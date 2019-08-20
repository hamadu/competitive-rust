// https://atcoder.jp/contests/agc024/tasks/agc024_d
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


fn doit(g: &Vec<Vec<usize>>, roots: Vec<usize>) -> (i64, i64) {
    let n = g.len();
    let mut deq = VecDeque::new();
    let mut ccnt = 1;
    if roots.len() == 1 {
        deq.push_back((roots[0], n));
    } else {
        deq.push_back((roots[0], roots[1]));
        deq.push_back((roots[1], roots[0]));
        ccnt = 2;
    }

    let mut depth = 0;
    loop {
        depth += 1;
        let mut next_deq = VecDeque::new();
        let mut max_children = 0;
        for (v, par) in deq {
            let mut cn = 0;
            for &t in &g[v] {
                if t == par {
                    continue;
                }
                cn += 1;
                next_deq.push_back((t, v));
            }
            max_children = max(cn, max_children);
        }
        if max_children == 0 {
            break;
        }
        ccnt *= max_children;
        deq = next_deq;
    }
    return (depth, ccnt);
}

fn main() {
    input! {
        n: usize,
        edges: [(usize1, usize1); n-1]
    };

    let mut g = vec![vec![]; n];
    for &(u, v) in &edges {
        g[u].push(v);
        g[v].push(u);
    }

    let mut dls = vec![];
    for i in 0..n {
        let (d, l) = doit(&g, vec![i]);
        dls.push((d, l));
    }
    for &(u, v) in &edges {
        let (d, l) = doit(&g, vec![u, v]);
        dls.push((d, l));
    }

    let mut depth = 100000;
    let mut leaf = 0;
    for (d, l) in dls {
        if d < depth || (d == depth && l < leaf) {
            depth = d;
            leaf = l;
        }
    }


    println!("{} {}", depth, leaf);
}
