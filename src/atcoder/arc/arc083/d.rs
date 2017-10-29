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
    let n: usize = read();
    let mut a: Vec<Vec<i64>> = vec![];

    for i in 0..n {
        let d: Vec<i64> = readnc();
        a.push(d);
    }

    let mut isok = true;
    let mut dec = 0;
    for i in 0..n {
        for j in 0..n {
            let mut required = true;
            for k in 0..n {
                if i == j || j == k || i == k {
                    continue
                }
                if a[i][j] > a[i][k] + a[k][j] {
                    isok = false;
                } else if a[i][j] == a[i][k] + a[k][j] {
                    required = false;
                }
            }
            if !required {
                dec += a[i][j];
            }
        }
    }

    let mut total = 0;
    for i in 0..n {
        for j in 0..n {
            total += a[i][j];

        }
    }

    if isok {
        println!("{}", (total-dec)/2);
    } else {
        println!("-1");
    }

}