// https://atcoder.jp/contests/agc021/tasks/agc021_e
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

    fn comb(&self, n: i64, r: i64) -> u64 {
        if n < 0 || r < 0 || r > n {
            return 0;
        }
        self.fact[n as usize] * self.invfact[r as usize] % MOD * self.invfact[(n - r) as usize] % MOD
    }

    fn perm(&self, n: usize, r: usize) -> u64 {
        if r < 0 || r > n {
            return 0;
        }
        self.fact[n] * self.invfact[n-r] % MOD
    }
}

fn main() {
    input! {
        n: i64, k: i64
    };

    let comb = Combination::new((n+k) as usize +10);
    let mut ways = 0;
    for p in 1..k+1 {
        let m = k-p;
        if p < m || p < n {
            continue;
        }

        let wb = k-(n-1)*2;
        let deadline = wb / 2;

        if p == m {
            let all = comb.comb(k-1, p);
            // (0,0) -> (k-1,-1-2*deadline)
            let p2 = k-2*deadline-2;
            let left = if p2 % 2 == 0 {
                comb.comb(k-1, p2 / 2)
            } else {
                0
            };
            ways += all + MOD - left;
        } else {
            let diff = p-m;
            let dd = deadline + diff / 2 + if wb % 2 == 1 { 1 } else { 0 };

            let all = comb.comb(k, p);
            // (0,0) -> (k, -2*dd-diff)
            let p2 = k-dd*2-diff;
            let left = if p2 % 2 == 0 {
                comb.comb(k, p2 / 2)
            } else {
                0
            };
            ways += all + MOD - left;

            debug!(p,m,diff,deadline,all,left);
        }
        ways %= MOD;
    }

    println!("{}", ways);
}
