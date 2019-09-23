// https://atcoder.jp/contests/keyence2019/tasks/keyence2019_f
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

    ($iter:expr, [ next / $t:tt ]) => {
        {
            let len = read_value!($iter, usize);
            (0..len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
        }
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
macro_rules! ifv {
    ($t:expr, $a:expr, $b: expr) => {
        if $t { $a } else { $b }
    }
}

#[allow(unused_macros)]
macro_rules! fill {
    ($t:expr, $v:expr) => {
        for i in 0..$t.len() {
            $t[i] = $v;
        }
    };
}

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

struct Combination {
    fact: Vec<i64>,
    invfact: Vec<i64>,
}

impl Combination {
    fn inv(a: i64) -> i64 {
        Combination::pow(a, MOD - 2)
    }

    fn pow(a: i64, p: i64) -> i64 {
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
            fact[i] = fact[i - 1] * (i as i64) % MOD;
        }

        invfact[upto - 1] = Combination::inv(fact[upto - 1]);
        invfact[0] = 1;
        for i in (1..upto - 1).rev() {
            invfact[i] = invfact[i + 1] * ((i + 1) as i64) % MOD;
        }

        Combination {
            fact: fact,
            invfact: invfact,
        }
    }

    fn comb(&self, n: usize, r: usize) -> i64 {
        if r < 0 || r > n {
            return 0;
        }
        self.fact[n] * self.invfact[r] % MOD * self.invfact[n - r] % MOD
    }

    fn perm(&self, n: usize, r: usize) -> i64 {
        self.fact[n] * self.invfact[n-r] % MOD
    }
}

const MOD: i64 = 1e9 as i64 + 7;

fn solve_bruteforce(h: usize, w: usize, k: usize) -> i64 {
    let comb = Combination::new(h+w+10);
    let mut total = 0i64;
    for x in 0..w+1 {
        for y in 0..h+1 {
            if x + y == 0 || x + y > k {
                continue;
            }
            let mut sc = (((x+1)*(y+1)) as i64)%MOD;
            sc *= comb.comb(x+y, x);
            sc %= MOD;
            sc *= comb.perm(w, x);
            sc %= MOD;
            sc *= comb.perm(h, y);
            sc %= MOD;

            let mut lptn = 0;
            let left = k - x - y;
            for nx in 0..left+1 {
                if nx+x > w {
                    continue;
                }
                let ny = left-nx;
                if ny+y > h {
                    continue;
                }
                let mut la = comb.comb(left, nx);
                la *= comb.perm(w-x, nx);
                la %= MOD;
                la *= comb.perm(h-y, ny);
                la %= MOD;
                lptn += la;
                lptn %= MOD;
            }

            sc *= lptn;
            sc %= MOD;

            total += sc;
            total %= MOD;

            debug!(x, y, (x+1)*(y+1), sc, total);
        }
    }
    total
}

fn solve(h: usize, w: usize, k: usize) -> i64 {
    let comb = Combination::new(h+w+10);
    let mut inner = 0i64;
    let mut outer = 0i64;
    let wh = (h * w) as i64 % MOD;
    for i in 1..k+1 {
        let left = (k-i+1) as i64;
        if i >= 2 {
            inner += 2 * wh % MOD * left % MOD * (i-1) as i64 % MOD * comb.perm(w+h-2, k-2) % MOD;
            inner %= MOD;
        }
        outer += ((h + w) as i64) * left % MOD * comb.perm(w+h-1, k-1) % MOD;
        outer %= MOD;
    }
    // debug!(inner, outer, comb.perm(h+w, k) * (k as i64));
    (inner + outer + comb.perm(h+w, k) * (k as i64) % MOD) % MOD
}

fn main() {
    input! {
        h: usize, w: usize, k: usize
    };

    // solve_bruteforce(h, w, k);

    println!("{}", solve(h, w, k));
}
