// https://atcoder.jp/contests/agc028/tasks/agc028_c
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

fn solve(v: Vec<(u64, u64)>) -> u64 {
    let mut edges = vec![];
    let n = v.len();
    for i in 0..n {
        edges.push((v[i].0, i, 0));
        edges.push((v[i].1, i, 1));
    }
    edges.sort();

    let mut ans = 0;
    let mut flag = vec![0; n];
    for i in 0..n {
        ans += edges[i].0;
        let idx = edges[i].1;
        flag[idx] |= 1 << edges[i].2;
    }

    let mut types = vec![0; 4];
    for i in 0..n {
        types[flag[i]] += 1;
    }

    assert!(types[0] == types[3]);

    if types[1] == n || types[2] == n {
        return ans;
    }
    if types[0] >= 1 {
        return ans;
    }

    let mut best = std::u64::MAX;
    for i in n - 2..n {
        let eidx = edges[i].1;
        for j in n..2 * n {
            if edges[j].1 != eidx {
                best = min(best, ans + edges[j].0 - edges[i].0);
                break;
            }
        }
    }
    return best;
}

fn main() {
    input! {
        n: usize,
        v: [(u64, u64); n]
    };

    let ans = solve(v);
    println!("{}", ans);
}
