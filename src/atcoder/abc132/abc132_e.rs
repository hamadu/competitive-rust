// https://atcoder.jp/contests/abc132/tasks/abc132_e
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
macro_rules! ifv {
    ($t:expr, $a:expr, $b: expr) => {
        if $t { $a } else { $b }
    }
}

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}


const INF: i64 = 1e18 as i64;

fn main() {
    input! {
        n: usize, m: usize,
        edges: [(usize1, usize1); m],
        s: usize1, t: usize1
    };

    let mut graph = vec![vec![]; 3*n];
    for e in edges {
        for w in 0..3 {
            let from = e.0 * 3 + w;
            let to = e.1 * 3 + (w + 1) % 3;
            graph[from].push(to);
        }
    }

    let mut dp = vec![INF; 3*n];
    dp[s*3] = 0;
    let mut deq = VecDeque::new();
    deq.push_back(s*3);
    while deq.len() >= 1 {
        let v = deq.pop_front().unwrap();
        let time = dp[v];
        for &to in &graph[v] {
            if dp[to] > time+1 {
                dp[to] = time+1;
                deq.push_back(to);
            }
        }
    }
    println!("{}", ifv!(dp[t*3] == INF, -1, dp[t*3]/3));
}
