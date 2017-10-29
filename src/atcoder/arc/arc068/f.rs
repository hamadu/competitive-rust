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

fn main() {
    let (n, k): (usize, usize) = read();

    let mut dp: Vec<Vec<Vec<i64>>> = vec![vec![vec![0; 2]; n+1]; n+1];
    dp[n][0][0] = 1;
    for i in (1..n+1).rev() {
        for j in (0..n+1).rev() {
            for k in 0..2 {
                let base = dp[i][j][k];
                if base == 0 {
                    continue
                }
                if j >= 1 && k == 0 {
                    dp[i][j-1][0] += base;
                    dp[i][j-1][0] %= MOD;
                }

                dp[i-1][j][0] += base;
                dp[i-1][j][0] %= MOD;

                dp[i-1][j+1][1] += base;
                dp[i-1][j+1][1] %= MOD;
            }
        }
    }

    let mut ans = (dp[1][n-k][0] + dp[1][n-k][1]) % MOD;
    for x in 1..n-k {
        ans *= 2;
        ans %= MOD;
    }
    println!("{}", ans);
}