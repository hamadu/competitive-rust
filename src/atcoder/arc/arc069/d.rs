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
    let s: Vec<char> = read::<String>().into_bytes().into_iter().map(|a| a as char).collect();

    let mut res = vec![false; n];
    let mut is_ok = false;
    for ptn in 0..4 {
        res[0] = ptn & 1 == 1;
        res[1] = ptn & 2 == 2;
        fill(&mut res, &s);
        if is_valid(&mut res, &s) {
            is_ok = true;
            break;
        }
    }

    if is_ok {
        for i in 0..n {
            print!("{}", if res[i] { 'S' } else { 'W' });
        }
        println!();
    } else {
        println!("-1");
    }
}

fn fill(animals: &mut Vec<bool>, statement: &Vec<char>) {
    let n = animals.len();
    for ri in 2..n {
        let li = ri-2;
        let i = ri-1;
        animals[ri] = animals[i] ^ animals[li] ^ (statement[i] == 'o');
    }
}

fn is_valid(animals: &Vec<bool>, statement: &Vec<char>) -> bool {
    let n = animals.len();
    for i in 0..n {
        let li = (i+n-1)%n;
        let ri = (i+1)%n;
        if animals[i] ^ (animals[li] == animals[ri]) ^ (statement[i] == 'x') {
            return false;
        }
    }
    true
}