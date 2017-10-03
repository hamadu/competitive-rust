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

const INF: i64 = 1_000_000_000_000;

fn main() {
    let n: i64 = read();
    let s: i64 = read();


    let b0 = solve_small(n, s);
    let b1 = solve_large(n, s);


    if min(b0, b1) == INF {
        println!("-1");
    } else {
        println!("{}", min(b0, b1));
    }
}

fn f(n: i64, b: i64) -> i64 {
    let mut nn = n;
    let mut sum = 0;
    while nn >= 1 {
        sum += nn % b;
        nn /= b;
    }
    sum
}

fn solve_small(n: i64, s: i64) -> i64 {
    for b in 2..n+1 {
        if b * b > n {
            break
        }
        if f(n, b) == s {
            return b;
        }
    }
    INF
}

fn solve_large(n: i64, s: i64) -> i64 {
    if n == s {
        return n + 1;
    }

    let am1 = n - s;
    let mut best = INF;
    for a in 1..(am1+1) {
        if a * a > am1 {
            break
        }
        if am1 % a != 0 {
            continue
        }
        let m = am1 / a + 1;
        if f(n, m) == s {
            best = min(best, m);
        }
    }
    best
}
