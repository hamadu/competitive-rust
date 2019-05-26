// https://atcoder.jp/contests/agc025/tasks/agc025_c
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


const INF: i64 = 100000000;

fn main() {
    input! {
        n: usize,
        lr: [(i64, i64); n]
    };

    let mut lrid: Vec<(usize, i64, i64)> = lr.iter().enumerate().map(|(idx, w)| (idx, w.0, w.1)).collect();
    let mut left = lrid.clone();
    left.sort_by_key(|w| w.2);
    let mut right = lrid.clone();
    right.sort_by_key(|w| -w.1);
    let mut lrtbl: Vec<Vec<(usize, i64, i64)>> = vec![left, right];

    let mut best = 0;

    // l to r
    for w in 0..2 {
        let mut used = vec![false; n];
        let mut dsum = 0;
        let mut last = vec![-INF, INF];
        let mut idx = vec![0, 0];
        let mut which = w;

        loop {
            while idx[which] < n {
                let i = lrtbl[which][idx[which]].0;
                if used[i] {
                    idx[which] += 1;
                    continue;
                }
                break;
            }

            if idx[which] == n {
                break;
            }
            if which == 0 {
                if lrtbl[0][idx[0]].2 > last[1] {
                    break;
                }
                dsum -= lrtbl[0][idx[0]].2 * 2;
            }
            if which == 1 {
                if lrtbl[1][idx[1]].1 < last[0] {
                    break;
                }
                dsum += lrtbl[1][idx[1]].1 * 2;
            }
            best = max(best, dsum);
            used[lrtbl[which][idx[which]].0] = true;
            idx[which] += 1;
            which ^= 1;
        }
   }


    println!("{}", best);
}
