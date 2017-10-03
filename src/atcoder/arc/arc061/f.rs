#![allow(unused_imports, unused_variables, dead_code)]
use std::io::*;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;

trait InputValue {
    fn parse(s: &str) -> Self;
}

fn read<T: InputValue>() -> T {
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    T::parse(&buf.trim())
}

fn readnc<T: InputValue>() -> Vec<T> {
    let mut vec = vec![];
    let line: String = read();
    for token in line.split_whitespace() {
        vec.push(T::parse(token));
    }
    vec
}

fn readn<T: InputValue>(n: usize) -> Vec<T> {
    let mut vec = vec![];
    for _ in 0..n {
        vec.push(read());
    }
    vec
}

macro_rules! parse_single_value {
    ($($t:ty),*) => {
        $(
            impl InputValue for $t {
                fn parse(s: &str) -> $t { s.parse().unwrap() }
            }
        )*
	}
}
parse_single_value!(i32, i64, f32, f64, usize, String);

macro_rules! parse_tuple {
	($($t:ident),*) => {
		impl<$($t),*> InputValue for ($($t),*) where $($t: InputValue),* {
			fn parse(s: &str) -> ($($t),*) {
				let mut tokens = s.split_whitespace();
				let t = ($($t::parse(tokens.next().unwrap())),*);
				t
			}
		}
	}
}
parse_tuple!(A, B);
parse_tuple!(A, B, C);

// ===

const MOD: i64 = 1000000007;

fn powmod(a: i64, p: i64, m: i64) -> i64 {
    let mut ret = 1i64;
    let mut aa = a;
    let mut pp = p;
    while pp >= 1 {
        if pp & 1 == 1 {
            ret *= aa;
            ret %= m;
        }
        aa = aa * aa % m;
        pp >>= 1;
    }
    ret
}

fn inv(a: i64, m: i64) -> i64 {
    powmod(a, m-2, m)
}

struct Combination {
    fact: Vec<i64>,
    invfact: Vec<i64>,
    modulo: i64
}

impl Combination {
    fn new(n: usize, modulo: i64) -> Self {
        let mut fact: Vec<i64> = vec![0; n];
        let mut invfact: Vec<i64> = vec![0; n];
        fact[0] = 1;
        for i in 1..n {
            fact[i] = fact[i-1] * i as i64 % modulo;
        }
        invfact[n-1] = inv(fact[n-1], modulo);
        for i in (0..n-1).rev() {
            invfact[i] = (invfact[i+1] * (i+1) as i64) % modulo;
        }

        Combination { fact: fact, invfact: invfact, modulo: modulo }
    }

    fn combination(&self, n: usize, k: usize) -> i64 {
        if n < k {
            return 0;
        }
        self.fact[n] * self.invfact[n-k] % self.modulo * self.invfact[k] % self.modulo
    }
}


fn main() {
    let (n, m, k): (usize, usize, usize) = read();
    let (m, k) = (min(m, k), max(m, k));

    let l = n + m + k;
    let comb = Combination::new(l + 10, MOD);

    let mut pow3: Vec<i64> = vec![0; l + 10];
    pow3[0] = 1;
    for i in 1..l+10 {
        pow3[i] = (pow3[i-1] * 3) % MOD;
    }


    let mut ans = 0;

    let mut mul = 1;
    let mut lid = 0;
    let mut rid = 0;
    for xl in n..l+1 {
        let left = xl - n;
        let right = l - xl;

        if left == 0 {
        } else if left <= m {
            mul = (mul * 2) % MOD;
        } else if left <= k {
            mul = (mul * 2 - comb.combination(left-1, lid) + MOD) % MOD;
            lid += 1;
        } else {
            mul = (mul * 2 - comb.combination(left-1, lid) - comb.combination(left-1, rid) + MOD + MOD) % MOD;
            lid += 1;
            rid += 1;
        }

        ans += comb.combination(xl-1, n-1) * mul % MOD * pow3[right] % MOD;
        ans %= MOD;
    }

    println!("{}", ans);
}