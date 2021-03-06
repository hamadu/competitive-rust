// https://atcoder.jp/contests/agc021/tasks/agc021_d
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

fn main() {
    input! {
        s: chars,
        c: usize
    };

    let n = s.len();

    let mut dp = vec![vec![vec![-1; c+1]; n+1]; n];
    dp[0][n-1][0] = 0;
    for i in 0..n {
        for j in (0..n).rev() {
            if i > j {
                continue;
            }
            for k in 0..c+1 {
                let base = dp[i][j][k];
                if base == -1 {
                    continue;
                }

                if i == j {
                    dp[0][n][k] = max(dp[0][n][k], base+1);
                    continue;
                }

                if s[i] == s[j] {
                    dp[i+1][j-1][k] = max(dp[i+1][j-1][k], base+2);
                } else if k+1 <= c {
                    dp[i+1][j-1][k+1] = max(dp[i+1][j-1][k+1], base+2);
                }
                if i+1 < n {
                    dp[i+1][j][k] = max(dp[i+1][j][k], base);
                }
                if j-1 >= 0 {
                    dp[i][j-1][k] = max(dp[i][j-1][k], base);
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..n+1 {
            for k in 0..c+1 {
                ans = max(ans, dp[i][j][k]);
            }
        }
    }
    println!("{}", ans);
}
