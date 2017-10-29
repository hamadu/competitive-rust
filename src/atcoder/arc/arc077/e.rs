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

fn dist(u: usize, v: usize, n: usize) -> usize {
    if u <= v {
        v - u
    } else {
        v + n - u
    }
}

fn add2(fr: usize, to: usize, d: i64, x2: &mut Vec<i64>) {
    let l = (to - fr + 1) as i64;
    x2[fr] += 1;
    x2[to+1] -= l + 1;
    x2[to+2] += l;

    x2[fr] += d - 1;
    x2[fr+1] -= d - 1;
    x2[to+1] -= d - 1;
    x2[to+2] += d - 1;
}

fn main() {
    let (n, m): (usize, usize) = read();
    let mut a: Vec<usize> = readnc();
    for i in 0..n {
        a[i] -= 1;
    }

    let mut sum = 0;
    for i in 1..n {
        sum += dist(a[i-1], a[i], m);
    }

    let mut x0 = vec![0; m+2];
    let mut x1 = vec![0; m+2];
    let mut x2 = vec![0; m+2];

    for i in 1..n {
        let fr = a[i-1];
        let to = a[i];
        if dist(fr, to, m) <= 1 {
            continue
        }
        let fr = (fr + 2) % m;
        if fr <= to {
            add2(fr, to, 1, &mut x2);
        } else {
            add2(fr, m-1, 1, &mut x2);
            add2(0, to, (m - fr + 1) as i64, &mut x2);
        }
    }

    x1[0] = x2[0];
    for i in 1..m+1 {
        x1[i] = x1[i-1] + x2[i];
    }
    x0[0] = x1[0];
    for i in 1..m+1 {
        x0[i] = x0[i-1] + x1[i];
    }

    let mut bestdec = 0;
    for i in 0..m {
        bestdec = max(bestdec, x0[i]);
    }
    println!("{}", sum as i64 - bestdec);
}