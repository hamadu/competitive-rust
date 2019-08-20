// https://atcoder.jp/contests/agc024/tasks/agc024_e
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
        n: usize, k: usize, MOD: usize
    };

   let mut comb = vec![vec![0; n+2]; n+2];
    for i in 0..n+2 {
        comb[i][0] = 1;
        comb[i][i] = 1;
        for j in 1..i {
            comb[i][j] = (comb[i-1][j-1] + comb[i-1][j]) % MOD;
        }
    }

    let mut dp = vec![vec![0; k+1]; n+2];
    for i in 0..k+1 {
        dp[1][i] = 1;
    }


    let mut dp_sum = vec![vec![0; k+2]; n+2];
    for i in 2..n+2 {
        for j in 0..k+1 {
            let p = dp_sum[i-1][j];
            dp_sum[i-1][j+1] = (p + dp[i-1][j]) % MOD;
        }

        for j in 0..k+1 {
            let mut total = 0;
            for w in 1..i {
                total += (MOD + dp_sum[w][k+1] - dp_sum[w][j+1]) % MOD * dp[i-w][j] % MOD * comb[i-2][w-1] % MOD;
                total %= MOD;
            }
            dp[i][j] = total;
            // dp[i][j] = sum(k, c(>j)) { dp[i-k][c] * dp[k][j] )
        }
    }

    println!("{}", dp[n+1][0]);
}
