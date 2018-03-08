#![allow(unused_imports, unused_variables, dead_code)]
use std::io::*;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;

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

const MOD: i64 = 1e9 as i64 + 7;

struct Solver {
    n: usize,
    k: usize,
    memo: Vec<Vec<i64>>,
    comb: Combination
}

impl Solver {
    fn dfs(&mut self, row: usize, head: usize) -> i64 {
        if self.memo[row][head] != -1 {
            return self.memo[row][head]
        }
        let mut ret = 0;
        if row + head >= 1 {
            if head >= 1 {
                ret += self.dfs(row, head-1);
            }
            if row > head {
                let left = head * self.k + (row - head) * (self.k - 1);
                ret += self.dfs(row-1, head) * self.comb.combination(left-1, self.k-2) % MOD;
            }
        } else {
            ret = 1;
        }
        ret %= MOD;
        self.memo[row][head] = ret;
        ret
    }
}

fn main() {
    let (n, k): (usize, usize) = read();
    if k == 1 {
        println!("1");
    } else {
        let memo = vec![vec![-1; n+1]; n+1];
        let comb = Combination::new(4000010, MOD);
        let mut solver = Solver { n: n, k: k, memo: memo, comb: comb };
        let mut ans = solver.dfs(n, n);
        for i in 1..n+1 {
            ans *= i as i64;
            ans %= MOD;
        }
        println!("{}", ans);
    }
}