#![allow(unused_imports, unused_variables, dead_code)]
use std::io::*;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;

trait InputValue {
    fn parse(s: &str) -> Self;
}

fn read<T: InputValue>() -> T {
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    T::parse(&buf.trim())
}

fn readvec<T: InputValue>() -> Vec<T> {
    let mut vec = vec![];
    let line: String = read();
    for token in line.split_whitespace() {
        vec.push(T::parse(token));
    }
    vec
}

fn readlines<T: InputValue>(n: usize) -> Vec<T> {
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
                ($($t::parse(tokens.next().unwrap())),*)
            }
        }
    }
}
parse_tuple!(A, B);
parse_tuple!(A, B, C);

// ===

fn main() {
    let (h, w, d): (usize, usize, usize) = read();
    let mut a: Vec<Vec<usize>> = vec![];

    let mut x = vec![0 as i32; h*w];
    let mut y = vec![0 as i32; h*w];
    for i in 0..h {
        let a: Vec<usize> = readvec();
        for j in 0..w {
            x[a[j]-1] = j as i32;
            y[a[j]-1] = i as i32;
        }
    }

    let mut jump = vec![vec![]; d];
    for i in 0..d {
        let mut j = i;
        let mut cost = 0;
        jump[i].push(cost);
        while j+d < h*w {
            cost += (x[j+d] - x[j]).abs();
            cost += (y[j+d] - y[j]).abs();
            jump[i].push(cost);
            j += d;
        }
    }

    let q = read();
    for i in 0..q {
        let (l, r): (usize, usize) = read();
        let bucket = (l-1)%d;
        let li = (l-1)/d;
        let ri = (r-1)/d;
        println!("{}", jump[bucket][ri] - jump[bucket][li]);
    }
}