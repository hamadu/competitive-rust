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
parse_tuple!(A, B, C, D);

// ===

const INF: usize = 1000000000;

fn decode(x: usize, y: usize, r: usize, c: usize) -> usize {
    if y == 0 {
        x
    } else if x == r {
        r + y
    } else if y == c {
        r + c + r - x
    } else if x == 0 {
        r + c + r + c - y
    }  else {
        INF
    }
}

fn main() {
    let (r, c, n): (usize, usize, usize) = read();
    let points: Vec<(usize, usize, usize, usize)> = readn(n);

    let mut pairs = vec![];
    for i in 0..n {
        let fpos = decode(points[i].0, points[i].1, r, c);
        let tpos = decode(points[i].2, points[i].3, r, c);
        if fpos == INF || tpos == INF {
            continue
        }
        pairs.push((fpos, i));
        pairs.push((tpos, i));
    }

    pairs.sort();

    let mut stk = VecDeque::new();
    let mut isok = true;
    let mut seen = vec![false; n];
    for i in 0..pairs.len() {
        let (_, i) = pairs[i];
        if !seen[i] {
            stk.push_front(i);
            seen[i] = true;
        } else {
            if stk.len() == 0 {
                isok = false;
                break
            }
            let last = stk.pop_front().unwrap();
            if last != i {
                isok = false;
                break
            }
        }
    }
    if stk.len() != 0 {
        isok = false;
    }

    if isok {
        println!("YES");
    } else {
        println!("NO");
    }
}