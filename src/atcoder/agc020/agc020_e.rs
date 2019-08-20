// https://atcoder.jp/contests/agc020/tasks/agc020_e
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


const MOD: u64 = 998244353;

type State = (u128, usize);

fn solve(s: State, dp: &mut HashMap<State, u64>) -> u64 {
    if dp.contains_key(&s) {
        return dp[&s];
    }
    let mut ans = 0;
    let n = s.1;

    if n == 0 {
        ans = 1;
    } else if n >= 1 {
        let mul = if s.0 & 1 == 1 { 2 } else { 1 };
        ans += mul * solve((s.0>>1, n-1), dp) % MOD;
        ans %= MOD;

        for v in 1..n {
            let mut mt = (1<<v)-1;
            for ct in 0..n/v {
                mt &= get(ct * v, (ct + 1) * v, s.0);
                if ct >= 1 {
                    ans += solve((mt, v), dp) * solve((get((ct + 1) * v, n, s.0), n - v * (ct + 1)), dp) % MOD;
                    ans %= MOD;
                }
            }
        }
    }

    dp.insert(s, ans);
    ans
}

fn get(l: usize, r: usize, v: u128) -> u128 {
    let f = (1<<(r-l))-1;
    (v >> l) & f
}

fn main() {
    input! {
        s: chars
    };
    let n = s.len();

    let mut dp = HashMap::new();
    let mut u = 0;
    for i in 0..n {
        if s[i] == '1' {
            u |= 1<<i;
        }
    }

    println!("{}", solve((u, n), &mut dp));
}
