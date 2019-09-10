// https://atcoder.jp/contests/diverta2019/tasks/diverta2019_e
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

const MAX: usize = 1<<20;
const MOD: i64 = 1e9 as i64 + 7;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut xorsum = vec![0; n+1];
    for i in 0..n {
        xorsum[i+1] = xorsum[i] ^ a[i];
    }


    let mut dp = vec![0; MAX];
    let mut dp_zero = vec![0; MAX];
    let mut last_zero = vec![0; MAX];

    let mut idx = n;
    let mut mode = xorsum[n];
    let mut zero_count = 0;
    if mode == 0 {
        for i in 1..MAX {
            dp_zero[i] = 1;
        }
    } else {
        dp[mode] = 1;
    }

    loop {
        let mut histories = vec![];
        while idx >= 0 && xorsum[idx] != 0 {
            histories.push(xorsum[idx]);
            idx -= 1;
        }

        histories.sort();
        let hn = histories.len();
        let mut fr = 0;
        while fr < hn {
            let mut to = fr;
            while to < hn && histories[to] == histories[fr] {
                to += 1;
            }
            let count = (to - fr) as i64;
            let num = histories[fr];

            let add_zero = zero_count - last_zero[num];
            last_zero[num] = zero_count;
            dp_zero[num] += add_zero * dp[num] % MOD;
            dp_zero[num] %= MOD;

            dp[num] += count * dp_zero[num] % MOD;
            dp[num] %= MOD;

            // debug!(num, last_zero[num], zero_count, dp_zero[num], dp[num]);

            fr = to;
        }

        while idx >= 1 && xorsum[idx] == 0 {
            idx -= 1;
            zero_count += 1;
        }

        if idx == 0 {
            break;
        }
    }

    let mut ans = 0;
    if mode == 0 {
        for w in 1..MAX {
            ans += dp[w];
            ans %= MOD;
        }
        let mut count = 1;
        for i in 1..n {
            if xorsum[i] == 0 {
                count *= 2;
                count %= MOD;
            }
        }
        ans += count;
        ans %= MOD;
    } else {
        ans = dp[mode];
    }
    println!("{}", ans);
}
