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
    let s: Vec<char> = read::<String>().chars().rev().collect::<Vec<char>>();
    let tokens: Vec<Vec<char>> = vec!["dream", "dreamer", "erase", "eraser"].into_iter().map(|x| x.chars().rev().collect::<Vec<char>>()).collect();

    let mut idx = 0;
    while idx < s.len() {
        let mut found = false;
        for t in &tokens {
            let n = t.len();
            if idx + n <= s.len() {
                let mut isok = true;
                for i in 0..n {
                    if s[idx+i] != t[i] {
                        isok = false;
                    }
                }
                if isok {
                    found = true;
                    idx += n;
                    break
                }
            }
        }
        if !found {
            break
        }
    }
    println!("{}", if idx == s.len() { "YES" } else { "NO" });
}