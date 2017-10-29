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
    let mut a: Vec<i64> = readnc();

    let mut max = 0;
    let mut inc_set: Vec<(usize, i64)> = vec![];
    inc_set.push((0, 0));

    for i in 0..n {
        if max < a[i] {
            max = a[i];
            inc_set.push((i, a[i]));
        }
    }

    a.sort();
    a.reverse();

    let ln = inc_set.len();
    let mut current = inc_set[ln-1].1;
    let mut head: usize = 0;
    let mut ans = vec![0; n];
    for i in (1..ln).rev() {
        let mut to = inc_set[i-1].1;
        let mut cnt = head as i64 * (inc_set[i].1 - to);
        while head < n && a[head] >= to {
            cnt += a[head] - to;
            head += 1;
        }
        ans[inc_set[i].0] = cnt;
    }

    for i in 0..n {
        println!("{}", ans[i]);
    }
}