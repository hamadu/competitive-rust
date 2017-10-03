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

const DY: [usize; 9] = [0, 0, 0, 1, 1, 1, 2, 2, 2];
const DX: [usize; 9] = [0, 1, 2, 0, 1, 2, 0, 1, 2];

fn main() {
    let (h, w, n): (usize, usize, usize) = read();
    let a: Vec<(usize, usize)> = readn(n);

    let mut blocks = HashSet::new();
    let mut seen = HashSet::new();
    for &(y, x) in &a {
        blocks.insert(y * (w+1) + x);
    }


    let mut ans: Vec<usize> = vec![0; 10];
    let mut total = 0;
    for &(y, x) in &a {
        for d in 0..9 {
            let bx = x + DX[d] - 1;
            let by = y + DY[d] - 1;
            if bx <= 1 || by <= 1 || bx >= w || by >= h {
                continue
            }
            let id = by * (w + 1) + bx;
            if seen.contains(&id) {
                continue
            }
            seen.insert(id);

            let mut counter = 0;
            for d2 in 0..9 {
                let cx = bx + DX[d2] - 1;
                let cy = by + DY[d2] - 1;
                let id = cy * (w + 1) + cx;
                if blocks.contains(&id) {
                    counter += 1;
                }
            }
            ans[counter] += 1;
            total += 1;
        }
    }
    ans[0] = (h - 2) * (w - 2) - total;

    for ai in ans {
        println!("{}", ai);
    }
}