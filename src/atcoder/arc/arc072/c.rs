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

const INF: i64 = 1e18 as i64;

fn solve(head: i64, a: &Vec<i64>) -> i64 {
    let n = a.len();
    let mut sum = (a[0] - head).abs();
    let mut psum = head;
    for i in 1..n {
        let tsum = psum + a[i];
        if psum < 0 {
            sum += max(1 - tsum, 0);
            psum = max(1, tsum);
        } else {
            sum += max(tsum + 1, 0);
            psum = min(-1, tsum);
        }
    }
    sum
}

fn main() {
    let n: usize = read();
    let mut a: Vec<i64> = readnc();

    let head = a[0];
    let mut ans = min(solve(1, &a), solve(-1, &a));
    if a[0] != 0 {
        ans = min(ans, solve(a[0], &a));
    }
    println!("{}", ans);
}