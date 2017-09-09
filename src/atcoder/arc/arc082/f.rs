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

struct Query {
    time: i32,
    initial: i32
}

fn main() {
    let x: i32 = read();
    let k: usize = read();
    let mut reverse_at: Vec<i32> = readnc();
    reverse_at.insert(0, 0);
    reverse_at.push(1000000010);

    let q: usize = read();
    let queries: Vec<(i32, i32)> = readn(q);
    let queries: Vec<Query> = queries.into_iter().map(|(a, b)| {
        Query { time: a, initial: b }
    }).collect();

    let mut min_sand = 0;
    let mut max_sand = x;
    let mut argmin = 0;
    let mut argmax = x;

    let mut answers: Vec<i32> = vec![0; q];

    let mut head: usize = 0;
    let mut incr = -1;
    for ti in 1..k+2 {
        let next_reverse = reverse_at[ti];
        let prev_reverse = reverse_at[ti-1];
        while head < q && queries[head].time < next_reverse {
            let q = &queries[head];
            let diff = q.time - prev_reverse;
            let at0= if q.initial < argmin {
                min_sand
            } else if argmax < q.initial {
                max_sand
            } else {
                min_sand + q.initial - argmin
            };

            let to = min(x, max(0, at0 + diff * incr));
            answers[head] = to;
            head += 1;
        }

        let diff = next_reverse - prev_reverse;
        let to_min_sand = min_sand + diff * incr;
        let to_max_sand = max_sand + diff * incr;

        if to_min_sand < 0 {
            argmin -= to_min_sand;
        }
        if to_max_sand > x {
            argmax -= to_max_sand - x;
        }
        min_sand = min(x, max(0, to_min_sand));
        max_sand = min(x, max(0, to_max_sand));
        if min_sand == max_sand {
            argmin = x;
            argmax = x;
        }
        incr *= -1;
    }

    for a in answers {
        println!("{}", a);
    }
}
