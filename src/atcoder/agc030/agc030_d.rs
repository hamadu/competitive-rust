// https://atcoder.jp/contests/agc030/tasks/agc030_d
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

const MOD: u64 = 1000000007;

fn pow(a: u64, p: u64, m: u64) -> u64 {
    let mut ret = 1;
    let mut aa = a;
    let mut p = p;
    while p >= 1 {
        if p & 1 == 1 {
            ret *= aa;
            ret %= m;
        }
        aa = (aa * aa) % m;
        p >>= 1;
    }
    ret
}

fn inv(a: u64, m: u64) -> u64 {
    pow(a, m - 2, m)
}

fn main() {
    input! {
        n: usize, q: usize,
        a: [u32; n],
        queries: [(usize1, usize1); q]
    };

    let mut dp = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            if a[i] > a[j] {
                dp[i][j] = 1;
            }
        }
    }

    let div2 = inv(2, MOD);

    for (u, v) in queries {
        let a = min(u, v);
        let b = max(u, v);

        let mut ati = vec![];
        let mut bti = vec![];
        let mut ita = vec![];
        let mut itb = vec![];
        for i in 0..n {
            ati.push(dp[a][i]);
            bti.push(dp[b][i]);
            ita.push(dp[i][a]);
            itb.push(dp[i][b]);
        }

        {
            let r = (ati[b] * div2 + bti[a] * div2) % MOD;
            dp[a][b] = r;
            dp[b][a] = r;
        }

        for i in 0..n {
            if i == b || i == a {
                continue;
            }
            let abi = (ati[i] * div2 + bti[i] * div2) % MOD;
            let iab = (ita[i] * div2 + itb[i] * div2) % MOD;
            dp[a][i] = abi;
            dp[b][i] = abi;
            dp[i][a] = iab;
            dp[i][b] = iab;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            ans += dp[i][j];
            ans %= MOD;
        }
    }
    ans *= pow(2, q as u64, MOD);
    ans %= MOD;

    println!("{}", ans);
}
