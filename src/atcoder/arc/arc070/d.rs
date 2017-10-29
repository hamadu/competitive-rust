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
    let (n, k): (usize, usize) = read();
    let mut a: Vec<usize> = readnc();

    let left = dodp(&a, k);
    a.reverse();
    let right = dodp(&a, k);
    a.reverse();

    let mut ans = 0;
    for i in 0..n {
        if a[i] >= k {
            continue
        }
        let mut rsum = vec![0; k+2];
        for r in 0..k {
            rsum[r+1] = rsum[r] + if right[n-i-1][r] { 1 } else { 0 };
        }

        let mut isok = false;
        for l in 0..k+1 {
            let from = if k <= a[i] + l { 0 } else { k - a[i] - l };
            let to = k - l;
            if left[i][l] && rsum[to] - rsum[from] >= 1 {
                isok = true;
                break
            }
        }
        if !isok {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn dodp(a: &Vec<usize>, k: usize) -> Vec<Vec<bool>> {
    let n = a.len();
    let mut dp: Vec<Vec<bool>> = vec![vec![false; k+1]; n+1];
    dp[0][0] = true;
    for i in 0..n {
        for j in 0..k+1 {
            if !dp[i][j] {
                continue
            }
            dp[i+1][j] = true;

            let tj = min(k, j + a[i]);
            dp[i+1][tj] = true;
        }
    }
    dp
}