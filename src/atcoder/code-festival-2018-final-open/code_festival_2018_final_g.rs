// https://atcoder.jp/contests/code-festival-2018-final-open/tasks/code_festival_2018_final_g
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

const INF: i64 = 1e18 as i64;

fn main() {
    input! {
        n: usize, m: usize,
        a: [i64; n]
    };
    let mut a = a;
    a.sort();

    let mut prefix = vec![0; n+1];
    for i in 0..n {
        prefix[i+1] = prefix[i] + a[i];
    }

    let mut dp = dvec!(INF; n+1, m+1);
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..m {
            let base = dp[i][j];
            if base == INF {
                continue;
            }
            let left_cage = m-j;
            let left_chicken = n-i;
            let max_num = min(left_chicken, if j == 0 { n } else { i/j });
            let min_num = (left_chicken+left_cage-1)/left_cage;
            // debug!(i, j, base, min_num, max_num);
            for k in min_num..max_num+1 {
                let ti = i+k;
                let tj = j+1;
                dp[ti][tj] = min(dp[ti][tj], base + (prefix[ti] - prefix[i]) * (k as i64));
            }
        }
    }

    println!("{}", dp[n][m]);
}
