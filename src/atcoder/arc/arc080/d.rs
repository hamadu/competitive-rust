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
    let (h, w): (usize, usize) = read();
    let n: usize = read();
    let a: Vec<usize> = readnc();

    let mut positions: Vec<(usize, usize)> = vec![];
    for i in 0..h {
        if i % 2 == 0 {
            for j in 0..w {
                positions.push((i, j));
            }
        } else {
            for j in (0..w).rev() {
                positions.push((i, j));
            }
        }
    }

    let mut ans: Vec<Vec<usize>> = vec![vec![0; w]; h];
    let mut idx = 0;
    for i in 0..n {
        for j in 0..a[i] {
            let (y, x) = positions[idx];
            ans[y][x] = i+1;
            idx += 1
        }
    }

    for i in 0..h {
        for j in 0..w {
            if j >= 1 {
                print!(" ");
            }
            print!("{}", ans[i][j]);
        }
        println!();
    }
}