// https://atcoder.jp/contests/agc022/tasks/agc022_e
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

const MOD: u64 = 1e9 as u64 + 7;

fn main() {
    input! {
        s: chars
    };
    let n = s.len();

    let mut dp = vec![vec![0; 8]; n+1];

    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..8 {
            let base = dp[i][j];
            let last = j & 1;
            let has_one = j & 2;
            let has_two = j & 4;

            for w in 0..2 {
                if s[i] == '1' && w == 0 {
                    continue;
                }
                if s[i] == '0' && w == 1 {
                    continue;
                }


                let mut tj = has_one | has_two | w;
                if i == 0 && w == 1 {
                    tj |= 2;
                }
                if i >= 1 && w == 1 && last == 1 {
                    if (i-1)%2 == 1 {
                        tj |= 2;
                    } else if (tj & 2) == 2 {
                        tj |= 4;
                    }
                }
                if i == n-1 && w == 1 && (tj & 2) == 2 {
                    tj |= 4;
                }
                dp[i+1][tj] += base;
                dp[i+1][tj] %= MOD;
            }
        }
    }

    println!("{}", (dp[n][6] +dp[n][7]) % MOD);
}
