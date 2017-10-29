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
    let (n, l): (usize, usize) = read();
    let tv: Vec<(f64, usize)> = readn(n);


    let mut deq: VecDeque<(f64, usize)> = VecDeque::new();
    let mut sum = 0;
    let mut total_value = 0.0;
    for i in 0..n {
        while sum + tv[i].1 > l {
            let (t, v) = deq.pop_front().unwrap();
            let trash = min(v, sum + tv[i].1 - l);
            sum -= trash;
            total_value -= trash as f64 * t;
            if v - trash >= 1 {
                deq.push_front((t, v - trash));
                break
            }
        }
        deq.push_back((tv[i].0, tv[i].1));
        sum += tv[i].1;
        total_value += tv[i].0 * tv[i].1 as f64;

        println!("{:.12}", total_value / l as f64);

        while deq.len() >= 2 {
            let (t0, v0) = deq.pop_back().unwrap();
            let (t1, v1) = deq.pop_back().unwrap();
            if t1 <= t0 {
                deq.push_back((t1, v1));
                deq.push_back((t0, v0));
                break
            }
            let vf0 = v0 as f64;
            let vf1 = v1 as f64;
            deq.push_back(((t0 * vf0 + t1 * vf1) / (vf0 + vf1), v0 + v1));
        }
    }
}