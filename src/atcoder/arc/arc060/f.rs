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

fn error_function(a: &Vec<u8>) -> Vec<usize> {
    let n = a.len();
    let mut err: Vec<usize> = vec![0; n+1];
    err[0] = n + 10;

    for i in 2..n+1 {
        let mut now = err[i-1];
        while now > 0 && a[i-1] != a[now] {
            now = err[now];
        }
        if a[i-1] == a[now] {
            err[i] = now + 1;
        } else {
            err[i] = 0;
        }
    }
    err
}

fn main() {
    let a: String = read();
    let a = a.into_bytes();

    let n = a.len();
    let prefix = error_function(&a);

    let mut ra = a;
    ra.reverse();

    let suffix = error_function(&ra);

    let cycle = if n % (n - prefix[n]) == 0 { n - prefix[n] } else { n };
    if cycle == n {
        println!("{}\n{}", 1, 1);
    } else if cycle == 1 {
        println!("{}\n{}", n, 1);
    } else {
        let mut cnt = n - 1;
        for pre in 1..n {
            let suf = n - pre;
            if (pre >= 2 && prefix[pre] >= 1 && pre % (pre - prefix[pre]) == 0) || (suf >= 2 && suffix[suf] >= 1 && suf % (suf - suffix[suf]) == 0) {
                cnt -= 1;
            }
        }
        println!("{}\n{}", 2, cnt);
    }
}