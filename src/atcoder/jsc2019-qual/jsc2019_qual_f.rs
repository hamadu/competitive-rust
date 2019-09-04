// https://atcoder.jp/contests/jsc2019-qual/tasks/jsc2019_qual_f
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

    fn comb(&self, n: u64, r: u64) -> u64 {
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

const MOD: u64 = 1e9 as u64 + 7;

fn doit(n: u64, less: u64, total: u64, max: u64, comb: &Combination) -> u64 {
    let mut ret = 0;
    for z in 0..less+1 {
        if total < max * z {
            break;
        }
        let part = comb.comb(total-max*z+n, n) * comb.comb(less, z) % MOD;
        if z % 2 == 0 {
            ret += part;
        } else {
            ret += MOD-part;
        }
        ret %= MOD;
    }
    ret
}

fn solve(n: u64, m: u64, x: u64, comb: &Combination) -> u64 {
    let all = comb.comb(x+n, n); // Σ(i=0 to x)comb.comb(i+n-1, n-1);
    let less = n-m;
    let more = m;

    let mut total_ng = 0;
    for k in 0..x {
        let lk = k;
        let rk = k+1;
        if lk + rk > x {
            continue;
        }
        if rk * more > x {
            continue
        }
        // L is no less than lk+1
        let mut w0 = doit(n, less, x-rk*more, lk+1, comb);
        // L is no less than lk
        let mut w1 = doit(n, less, x-rk*more, lk, comb);

        // L is exactly equal to lk
        total_ng += w0 + MOD - w1;
        total_ng %= MOD;

    }
    total_ng *= comb.comb(less+more, less);
    total_ng %= MOD;
    (all + MOD - total_ng) % MOD
}

fn main() {
    input! {
        n: u64, m: u64, l: u64, r: u64
    };

    let comb = Combination::new(650000);
    let left = solve(n, m, l-1, &comb);
    let right = solve(n, m, r, &comb);

    // debug!(left, right);
    println!("{}", (right + MOD - left) % MOD);
}
