// https://atcoder.jp/contests/agc028/tasks/agc028_b
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

struct Combination {
    fact: Vec<u64>,
    invfact: Vec<u64>,
}

impl Combination {
    fn inv(a: u64) -> u64 {
        Combination::pow(a, MOD - 2)
    }

    fn pow(a: u64, p: u64) -> u64 {
        let mut p = p;
        let mut aa = a;
        let mut ret = 1;
        while p >= 1 {
            if p & 1 == 1 {
                ret *= aa;
                ret %= MOD;
            }
            p >>= 1;
            aa = aa * aa % MOD;
        }
        ret
    }

    fn new(upto: usize) -> Self {
        let mut fact = vec![0; upto];
        let mut invfact = vec![0; upto];

        fact[0] = 1;
        for i in 1..upto {
            fact[i] = fact[i - 1] * (i as u64) % MOD;
        }

        invfact[upto - 1] = Combination::inv(fact[upto - 1]);
        invfact[0] = 1;
        for i in (1..upto - 1).rev() {
            invfact[i] = invfact[i + 1] * ((i + 1) as u64) % MOD;
        }

        Combination {
            fact: fact,
            invfact: invfact,
        }
    }

    fn comb(&self, n: usize, r: usize) -> u64 {
        if r < 0 || r > n {
            return 0;
        }
        self.fact[n] * self.invfact[r] % MOD * self.invfact[n - r] % MOD
    }
}

fn main() {
    input! {
        n: usize,
        a: [u64; n]
    };
    let ans = solve(a);
    println!("{}", ans);
}

fn solve(a: Vec<u64>) -> u64 {
    let n = a.len();
    let comb = Combination::new(100010);
    let mut prob_len = vec![0; n + 1];
    let mut prob_len_sum = vec![0; n + 1];
    for w in 1..n + 1 {
        prob_len[w] = Combination::inv(w as u64);
        let s = prob_len_sum[w - 1] + prob_len[w];
        prob_len_sum[w] = s % MOD;
    }

    let mut ans = 0;
    for i in 0..n {
        let l = i + 1;
        let r = n - i;
        let rsum = (prob_len_sum[l] + prob_len_sum[r] + MOD - 1) % MOD;
        ans += rsum * a[i] % MOD;
    }

    ans %= MOD;
    ans *= comb.fact[n];
    ans %= MOD;

    ans
}
