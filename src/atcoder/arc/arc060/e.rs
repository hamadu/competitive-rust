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
    let x: Vec<i32> = readnc();
    let L: i32 = read();
    let q: usize = read();
    let queries: Vec<(usize, usize)> = readn(q);

    let mut right: Vec<Vec<usize>> = vec![vec![0; n]; 18];

    {
        let mut head = 0;
        for i in 0..n {
            while head+1 < n && x[head+1] - x[i] <= L {
                head += 1;
            }
            right[0][i] = head;
        }
    }

    for k in 1..right.len() {
        for i in 0..n {
            right[k][i] = right[k-1][right[k-1][i]];
        }
    }

    for &(a, b) in &queries {
        let mut len = 0;
        let mut head = min(a - 1, b - 1);
        let to = max(a - 1, b - 1);
        for k in (0..right.len()).rev() {
            if right[k][head] < to {
                len += 1<<k;
                head = right[k][head];
            }
        }
        if head != to {
            len += 1;
        }
        println!("{}", len);
    }
}

