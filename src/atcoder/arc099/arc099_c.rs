// https://atcoder.jp/contests/arc099/tasks/arc099_c
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

fn dfs(now: usize, col: i32, colors: &mut Vec<i32>, tbl: &Vec<Vec<i32>>) -> bool {
    if colors[now] != 0 {
        return colors[now] == col;
    }
    colors[now] = col;

    let n = tbl.len();
    for i in 0..n {
        if tbl[now][i] == 1 {
            if !dfs(i, -col, colors, tbl) {
                return false;
            }
        }
    }
    true
}

fn main() {
    input! {
        n: usize, m: usize,
        edges: [(usize1, usize1); m]
    };

    let mut tbl = dvec!(1; n, n);
    for &e in &edges {
        tbl[e.0][e.1] = 0;
        tbl[e.1][e.0] = 0;
    }
    for i in 0..n {
        tbl[i][i] = 0;
    }

    let mut colors = vec![0; n];
    let mut cid = 1;
    let mut is_biparate = true;
    for i in 0..n {
        if colors[i] == 0 {
            let ok = dfs(i, cid, &mut colors, &tbl);
            cid += 1;
            if !ok {
                is_biparate = false;
                break;
            }
        }
    }
    if !is_biparate {
        println!("-1");
        return;
    }

    let mut pairs = dvec!(0; (cid - 1) as usize, 2);
    for i in 0..n {
        let pid = (colors[i].abs() as usize) - 1;
        let which = if colors[i] > 0 { 0 } else { 1 };
        pairs[pid][which] += 1;
    }

    let pl = pairs.len();
    let mut dp = dvec!(false; pl+1, n+1);
    dp[0][0] = true;
    for i in 0..pl {
        for j in 0..n+1 {
            if !dp[i][j] {
                continue;
            }
            dp[i+1][j+pairs[i][0]] = true;
            dp[i+1][j+pairs[i][1]] = true;
        }
    }

    let mut best = p2(n);
    for i in 0..n+1 {
        if dp[pl][i] {
            best = min(best, p2(i) + p2(n-i));
        }
    }
    println!("{}", best);
}


fn p2(x: usize) -> usize {
    if x <= 1 {
        0
    } else {
        x * (x-1) / 2
    }
}
