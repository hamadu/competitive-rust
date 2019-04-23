// https://atcoder.jp/contests/tenka1-2019/tasks/tenka1_2019_d
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

const MOD: u64 = 998244353;
const LEN: usize = 90000;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut dp: Vec<Vec<u64>> = vec![vec![0; LEN + 1]; 301];
    dp[0][0] = 3;
    for i in 0..n {
        for s in 0..LEN + 1 {
            let base = dp[i][s];
            if base == 0 {
                continue;
            }
            dp[i + 1][s] += base * 2;
            dp[i + 1][s] %= MOD;

            dp[i + 1][s + a[i]] += base;
            dp[i + 1][s + a[i]] %= MOD;
        }
    }

    let mut dp2: Vec<Vec<u64>> = vec![vec![0; LEN + 1]; 301];
    dp2[0][0] = 6;
    for i in 0..n {
        for s in 0..LEN + 1 {
            let base = dp2[i][s];
            if base == 0 {
                continue;
            }
            dp2[i + 1][s] += base;
            dp2[i + 1][s] %= MOD;

            dp2[i + 1][s + a[i]] += base;
            dp2[i + 1][s + a[i]] %= MOD;
        }
    }

    let mut total = 1;
    let mut two = 3;
    let mut tlen = 0;
    for i in 0..n {
        total *= 3;
        total %= MOD;
        two *= 2;
        two %= MOD;
        tlen += a[i];
    }
    total += (MOD - two);
    total %= MOD;

    for i in 1..tlen + 1 {
        let ab = tlen - i;
        if ab <= i {
            total += MOD - dp[n][i];
            total += MOD + dp2[n][i];
            total %= MOD;
        }
    }

    println!("{}", total);
}
