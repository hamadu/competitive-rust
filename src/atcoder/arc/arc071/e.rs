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
parse_tuple!(A, B, C, D);

// ===

fn compute_partial_sum(s: String) -> Vec<i32> {
    let n = s.len();
    let mut sum = vec![0; n+1];

    let s = s.into_bytes();
    for i in 0..n {
        if s[i] == 'A' as u8 {
            sum[i+1] = sum[i] + 1;
        } else {
            sum[i+1] = sum[i] + 2;
        }
    }
    sum
}

fn main() {
    let s: String = read();
    let t: String = read();

    let s = compute_partial_sum(s);
    let t = compute_partial_sum(t);

    let q: usize = read();
    for i in 0..q {
        let (a, b, c, d): (usize, usize, usize, usize) = read();

        if (s[b] - s[a-1]) % 3 == (t[d] - t[c-1]) % 3 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}