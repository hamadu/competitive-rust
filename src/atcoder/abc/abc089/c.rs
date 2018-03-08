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

fn readvec<T: InputValue>() -> Vec<T> {
    let mut vec = vec![];
    let line: String = read();
    for token in line.split_whitespace() {
        vec.push(T::parse(token));
    }
    vec
}

fn readlines<T: InputValue>(n: usize) -> Vec<T> {
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

fn idx(a: u8) -> usize {
    a as usize - 'A' as usize
}

fn main() {
    let n: usize = read();
    let s: Vec<String> = readlines(n);

    let mut v: Vec<u64> = vec![0; 26];
    for si in s {
        let a = si.into_bytes()[0];
        v[idx(a)] += 1;
    }

    let march = "MARCH".to_string().into_bytes();
    let mut sum: u64 = 0;
    for i in 0..5 {
        for j in i+1..5 {
            for k in j+1..5 {
                let ii = idx(march[i]);
                let jj = idx(march[j]);
                let kk = idx(march[k]);
                sum += v[ii] * v[jj] * v[kk];
            }
        }
    }
    println!("{}", sum);
}