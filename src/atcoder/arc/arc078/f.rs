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
    let (n, m): (usize, usize) = read();
    let mut table: Vec<Vec<i32>> = vec![vec![0; n]; n];
    for i in 0..m {
        let (a, b, c): (usize, usize, i32) = read();
        table[a-1][b-1] = c;
        table[b-1][a-1] = c;
    }

    let mut induced_value = vec![0; 1<<n];
    for p in 0..1<<n {
        for i in 0..n {
            for j in i+1..n {
                if table[i][j] >= 1 && p & (1<<i) >= 1 && p & (1<<j) >= 1 {
                    induced_value[p] += table[i][j];
                }
            }
        }
    }

    let all = (1<<(n-1))-1;
    let mut dp = vec![vec![-1; n]; all+1];
    for p in 0..1<<(n-1) {
        let tp = p | 1;
        dp[tp][0] = induced_value[tp];
    }

    let mut best = 0;
    for p in 0..1<<(n-1) {
        for now in 0..n {
            if dp[p][now] == -1 {
                continue
            }
            let base = dp[p][now];

            let sub = all ^ p;
            let mut left = sub;
            while left >= 1 {
                for to in 0..n-1 {
                    if left & (1<<to) == 0 || table[now][to] == 0 {
                        continue
                    }
                    dp[p|left][to] = max(dp[p|left][to], base + induced_value[left] + table[now][to])
                }
                left = (left - 1) & sub;
            }

            if table[now][n-1] >= 1 {
                best = max(best, base + induced_value[sub | (1<<(n-1))] + table[now][n-1])
            }
        }
    }

    let mut total = 0;
    for i in 0..n {
        for j in i+1..n {
            total += table[i][j];
        }
    }

    println!("{}", total-best);
}