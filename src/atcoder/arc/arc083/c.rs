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
parse_tuple!(A, B, C, D, E, F);

// ===

fn build_table(a: usize, b: usize, mul: usize, f: usize) -> Vec<bool> {
    let mut table = vec![false; f+1];
    table[0] = true;
    for i in 0..f+1 {
        if table[i] {
            if i + a * mul <= f {
                table[i+a*mul] = true;
            }
            if i + b * mul <= f {
                table[i+b*mul] = true;
            }
        }
    }
    table
}

fn main() {
    let (a, b, c, d, e, f): (usize, usize, usize, usize, usize, usize) = read();

    let water_table = build_table(a, b, 100, f);
    let sugar_table = build_table(c, d, 1, f);

    let mut back_sugar_table = vec![0; f+1];
    let mut last = 0;
    for i in 0..f+1 {
        if sugar_table[i] {
            last = i;
        }
        back_sugar_table[i] = last;
    }

    // (a, b) := a/b
    let mut best_rate = (0, 1);
    let mut best_total = 0;
    let mut best_sugar = 0;

    for w in 1..f+1 {
        if water_table[w] {
            let max_sugar = min(w * e / 100, f - w);
            let sugar = back_sugar_table[max_sugar];
            if best_rate.0 * (w + sugar) <= best_rate.1 * 100 * sugar {
                best_rate = (100 * sugar, w + sugar);
                best_total = w + sugar;
                best_sugar = sugar;
            }
        }
    }

    println!("{} {}", best_total, best_sugar);
}