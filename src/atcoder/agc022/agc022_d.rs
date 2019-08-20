// https://atcoder.jp/contests/agc022/tasks/agc022_d
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


const INF: i64 = 1e18 as i64;

fn main() {
    input! {
        n: usize, l: i64,
        x: [i64; n],
        t: [i64; n]
    };

    assert!(n <= 3000);

    let lp = l*2;

    // 0: left-left
    // 1: left-right (right-left)
    // 2: right-right
    let mut costs = vec![vec![0; 3]; n];
    for i in 0..n {
        // -> [*]
        let time_l = x[i]*2;
        // [*] <-
        let time_r = lp-x[i]*2;

        // l-r
        costs[i][1] = (t[i] + lp - 1) / lp * lp;

        // l-l
        costs[i][0] = time_r + (max(0, t[i]-time_r)+lp-1)/lp*lp;

        // r-r
        costs[i][2] = time_l + (max(0, t[i]-time_l)+lp-1)/lp*lp;
    }


    let mut dp = vec![vec![INF; n+1]; n+1];
    dp[0][0] = 0;

    let mut ans = INF;
    for i in 0..n {
        let d = x[i] - if i >= 1 { x[i-1] } else { 0 };
        for w in 0..n+1 {
            let wi = w as i64;
            let base = dp[i][w];
            if base == INF {
                continue;
            }
            let to = base + (wi+1) * d * 2;
            if i < n-1 {
                if w >= 1 {
                    dp[i+1][w-1] = min(dp[i+1][w-1], to + costs[i][0]);
                }
                dp[i+1][w] = min(dp[i+1][w], to + costs[i][1]);
                dp[i+1][w+1] = min(dp[i+1][w+1], to + costs[i][2]);
            } else {
                let nd = (l-x[i]) * 2;
                // close
                ans = min(ans, to + costs[i][0] + wi * nd);

                // pass
                ans = min(ans, to + costs[i][1] + (wi + 1) * nd);

                // open
                ans = min(ans, to + costs[i][2] + (wi + 2) * nd);
            }
        }
    }

    println!("{}", ans);
}
