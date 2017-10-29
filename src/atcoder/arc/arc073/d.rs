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

fn main() {
    let (n, w): (usize, usize) = read();
    let wv: Vec<(usize, i32)> = readn(n);

    let mw = n * 3;
    let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![vec![-1; mw+1]; n+1]; n+1];
    dp[0][0][0] = 0;

    let basew = wv[0].0;
    for i in 0..n {
        let wi = wv[i].0 - basew;
        for j in 0..n {
            for k in 0..mw+1 {
                let base = dp[i][j][k];
                if base == -1 {
                    continue
                }
                dp[i+1][j][k] = max(dp[i+1][j][k], base);

                let tj = j+1;
                let tk = k+wi;
                dp[i+1][tj][tk] = max(dp[i+1][tj][tk], base + wv[i].1);
            }
        }
    }

    let mut ans = 0;
    for j in 0..n+1 {
        for k in 0..mw+1 {
            let weight = j * basew + k;
            if weight <= w {
                ans = max(ans, dp[n][j][k]);
            }
        }
    }
    println!("{}", ans);
}