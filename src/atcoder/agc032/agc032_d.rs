// https://atcoder.jp/contests/agc032/tasks/agc032_d
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

const INF: i64 = 1e18 as i64;

fn main() {
    input! {
        n: usize, a: i64, b: i64,
        p: [usize1; n]
    };

    let mut dp = vec![vec![INF; n + 1]; 2];
    dp[0][n] = 0;

    for i in 0..n {
        let mut upper = 0;
        let mut lower = 0;
        for j in 0..i {
            if p[i] < p[j] {
                upper += 1;
            } else {
                lower += 1;
            }
        }

        let fr = i % 2;
        let to = 1 - fr;
        for j in 0..n + 1 {
            dp[to][j] = INF;
        }

        for j in 0..n + 1 {
            if dp[fr][j] == INF {
                continue;
            }
            let base = dp[fr][j];
            if j == n {
                dp[to][i] = min(dp[to][i], base + upper * a + lower * b);
            } else {
                if p[j] < p[i] {
                    dp[to][i] = min(dp[to][i], base);
                }
            }
            if j == n {
                dp[to][j] = min(dp[to][j], base);
            } else {
                if p[i] > p[j] {
                    dp[to][j] = min(dp[to][j], base + a);
                } else {
                    dp[to][j] = min(dp[to][j], base + b);
                }
            }
        }
    }
    let mut ans = INF;
    for i in 0..n {
        ans = min(ans, dp[n % 2][i]);
    }
    println!("{}", ans);
}
