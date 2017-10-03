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

const MOD: i64 = 1e9 as i64 + 7;

fn powmod(a: i64, p: usize, m: i64) -> i64 {
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

fn inv(a: i64, m: usize) -> i64 {
    powmod(a, m-2, m as i64)
}

fn main() {
    let (n, k): (usize, i64) = read();

    let ans = 0;

    let mut divisors = vec![];
    for d in 1..n+1 {
        let d2 = d * d;
        if d2 == n {
            divisors.push(d);
            break;
        } else if d2 > n {
            break;
        }
        if n % d == 0 {
            divisors.push(d);
            divisors.push(n / d);
        }
    }
    divisors.sort();

    let dn = divisors.len();

    let mut dp = vec![0; dn];
    for i in 0..dn {
        let d = divisors[i];
        let mut sum = powmod(k, (d + 1) / 2, MOD);
        for j in 0..i {
            if d % divisors[j] == 0 {
                sum += MOD - dp[j]
            }
        }
        sum %= MOD;
        dp[i] = sum;
    }

    let mut ans = 0;
    for i in 0..dn {
        let d = divisors[i] as i64;
        if d % 2 == 0 {
            ans += dp[i] * (d / 2) % MOD;
        } else {
            ans += dp[i] * d % MOD;
        }
        ans %= MOD;
    }
    println!("{}", ans);
}

