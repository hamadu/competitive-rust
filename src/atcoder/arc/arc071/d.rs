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

fn compute(a: &Vec<i64>) -> i64 {
    let n = a.len();
    let mut diff = vec![];
    for i in 1..n {
        diff.push((a[i] - a[i-1]) % MOD);
    }

    let mut sum = 0;
    let dn = n-1;
    for i in 0..dn {
        let l = i+1;
        let r = dn-i;
        sum += diff[i] * (l as i64) % MOD * (r as i64) % MOD;
        sum %= MOD;
    }
    sum
}

fn main() {
    let (n, m): (usize, usize) = read();
    let col: Vec<i64> = readnc();
    let row: Vec<i64> = readnc();
    println!("{}", compute(&col) * compute(&row) % MOD);
}
