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


fn main() {
    let n: usize = read();
    let s: String = read();


    let mut dp: Vec<Vec<i64>> = vec![vec![0; n+1]; 2];
    dp[0][0] = 1;

    for i in 0..n {
        let fr = i % 2;
        let to = 1 - fr;
        for k in 0..n+1 {
            dp[to][k] = 0;
        }

        for k in 0..n+1 {
            let base = dp[fr][k];
            if base == 0 {
                continue;
            }

            let tk = max(k as i32 - 1, 0) as usize;
            dp[to][tk] += base;
            dp[to][tk] %= MOD;

            if k + 1 <= n {
                dp[to][k+1] += base * 2 % MOD;
                dp[to][k+1] %= MOD;
            }
        }
    }

    let total = dp[n%2][s.len()];
    let inv2 = inv(2, MOD);
    println!("{}", total * powmod(inv2, s.len() as i64, MOD) % MOD);
}