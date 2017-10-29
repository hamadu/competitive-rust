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

fn solve0(xy: &Vec<(i64, i64)>) -> i64 {
    // fix rmin and bmax

    let n = xy.len();
    if n == 1 {
        return 0
    }

    let mut rmin = xy[0].0;
    let mut rmax = xy[0].0;
    let mut bmin = xy[0].1;
    let mut bmax = xy[0].1;

    let mut bi = 0;
    for i in 0..n {
        if bmax <= xy[i].1 {
            bmax = xy[i].1;
            bi = i;
        }
    }
    rmin = min(rmin, xy[bi].0);
    rmax = max(rmax, xy[bi].0);

    for i in 0..n {
        if i == 0 || i == bi {
            continue
        }
        bmin = min(bmin, xy[i].1);
        rmax = max(rmax, xy[i].0);
    }

    (rmax - rmin) * (bmax - bmin)
}

fn solve1(xy: &Vec<(i64, i64)>) -> i64 {
    // fix rmin and rmax
    let n = xy.len();
    if n == 1 {
        return 0
    }

    let mut rmin = xy[0].0;
    let mut rmax = xy[0].0;
    let mut bmin = xy[0].1;
    let mut bmax = xy[0].1;

    let mut bi = 0;
    for i in 1..n {
        if rmax <= xy[i].1 {
            rmax = xy[i].1;
            bi = i;
        }
    }
    bmin = min(bmin, xy[bi].0);
    bmax = max(bmax, xy[bi].0);

    if n == 2 {
        return (rmax - rmin) * (bmax - bmin)
    }

    let mut pairs = vec![];
    for i in 0..n {
        if i != 0 && i != bi {
            pairs.push(xy[i]);
        }
    }

    let pn = pairs.len();

    let mut right_min = 1e11 as i64;
    let mut right_max = 0;
    let mut best_blue = (0, 1e11 as i64);
    for i in 0..pn {
        let bmin = min(min(bmin, pairs[i].0), right_min);
        let bmax = max(max(bmax, right_max), pairs[pn-1].0);
        if best_blue.1 - best_blue.0 >= bmax - bmin {
            best_blue = (bmin, bmax);
        }
        right_min = min(right_min, pairs[i].1);
        right_max = max(right_max, pairs[i].1);
    }

    (rmax - rmin) * (best_blue.1 - best_blue.0)
}

fn main() {
    let n: usize = read();
    let mut xy: Vec<(i64, i64)> = readn(n);

    for i in 0..n {
        if xy[i].1 < xy[i].0 {
            xy[i] = (xy[i].1, xy[i].0);
        }
    }
    xy.sort();

    // println!("{:?}", xy);
    println!("{}", min(solve0(&xy), solve1(&xy)));
}