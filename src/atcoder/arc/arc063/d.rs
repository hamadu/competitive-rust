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
    let (n, t): (usize, usize) = read();
    let a: Vec<i32> = readnc();

    let mut best = 0;
    let mut min_price = 1000000000;
    for i in 0..n {
        let profit = a[i] - min_price;
        best = max(best, profit);
        min_price = min(min_price, a[i]);
    }

    let mut ans = 0;
    let mut head = 0;
    while head < n {
        let mut min_price = a[head];
        let mut min_price_count = 0;
        let mut max_price_count = 0;
        while head < n && min_price <= a[head] {
            if a[head] == min_price {
                min_price_count += 1;
            }
            if a[head] == min_price + best {
                max_price_count += 1;
            }
            head += 1;
        }
        ans += min(min_price_count, max_price_count);
    }
    println!("{}", ans);
}