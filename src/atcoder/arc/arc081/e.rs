#![allow(unused_imports, unused_variables, dead_code)]
use std::io::*;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;

pub trait InputValue {
    fn parse(s: &str) -> Self;
}

pub fn read<T: InputValue>() -> T {
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    T::parse(&buf.trim())
}

pub fn readnc<T: InputValue>() -> Vec<T> {
    let mut vec = vec![];
    let line: String = read();
    for token in line.split_whitespace() {
        vec.push(T::parse(token));
    }
    vec
}

pub fn readn<T: InputValue>(n: usize) -> Vec<T> {
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
    let a: String = read();
    let a: Vec<usize> = a.into_bytes().into_iter().map(|x| (x - 'a' as u8) as usize).collect();
    let n = a.len();

    let mut next_char: Vec<Vec<usize>> = vec![vec![n+1; 26]; n+1];
    let mut next: Vec<Vec<usize>> = vec![vec![n+1; n+2]; 20];

    let mut last: Vec<usize> = vec![n+1; 26];
    for i in (0..n).rev() {
        last[a[i]] = i + 1;
        next[0][i] = 0;
        for k in 0..26 {
            next_char[i][k] = last[k];
            next[0][i] = max(next[0][i], last[k]);
        }
    }

    for p in 1..20 {
        for i in 0..n {
            next[p][i] = next[p-1][next[p-1][i]];
        }
    }

    let req_length = length_from(0, n, &next) + 1;
    let mut answer: Vec<char> = vec![];

    let mut head: usize = 0;
    for i in 0..req_length {
        for j in 0..26 {
            if !can_go(next_char[head][j], n, req_length - 1 - i, &next) {
                answer.push(('a' as u8 + j as u8) as char);
                head = next_char[head][j];
                break;
            }
        }
    }

    println!("{}", answer.into_iter().collect::<String>());
}


fn can_go(from: usize, to: usize, req: usize, next: &Vec<Vec<usize>>) -> bool {
    let mut head = from;
    for p in (0..20).rev() {
        if req & (1<<p) >= 1 {
            head = next[p][head];
        }
    }
    head <= to
}


fn length_from(from: usize, to: usize, next: &Vec<Vec<usize>>) -> usize {
    let mut head = from;
    let mut total = 0;
    for p in (0..20).rev() {
        if next[p][head] <= to {
            total += 1 << p;
            head = next[p][head];
        }
    }
    total
}
