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

fn ok(a: Vec<Vec<i32>>) -> bool {
    let c1 = a[0][1] - a[0][0];
    let c2 = a[0][2] - a[0][1];
    let r1 = a[1][0] - a[0][0];
    let r2 = a[2][0] - a[1][0];

    for i in 0..3 {
        if a[i][1] - a[i][0] != c1 {
            return false;
        }
        if a[i][2] - a[i][1] != c2 {
            return false;
        }
        if a[1][i] - a[0][i] != r1 {
            return false;
        }
        if a[2][i] - a[1][i] != r2 {
            return false;
        }
    }
    true
}

fn main() {
    let mut a: Vec<Vec<i32>> = vec![];
    for i in 0..3 {
        a.push(readvec());
    }

    if (ok(a)) {
        println!("Yes");
    } else {
        println!("No");
    }



}