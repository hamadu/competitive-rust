#![allow(unused_imports, unused_variables, dead_code)]
use std::io::*;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;

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
    let n: usize = read();
    let a: Vec<i32> = readn(n);
    let mut a: Vec<(i32, usize)> = a.into_iter().zip(0..n).collect();
    a.sort();

    let mut diff = 0;
    for i in 0..n {
        if a[i].1 % 2 != i % 2 {
            diff += 1;
        }
    }

    println!("{}", diff / 2);
}