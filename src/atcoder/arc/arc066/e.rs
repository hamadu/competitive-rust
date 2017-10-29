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
    let n: usize = read();
    let cr: Vec<String> = readnc();

    let mut a: Vec<i64> = vec![];
    a.push(i64::parse(&cr[0]));

    for i in 1..n {
        let p = cr[i*2-1].to_string();
        let v = i64::parse(&cr[i*2]);
        if p == "+" {
            a.push(v);
        } else {
            a.push(-v);
        }
    }

    let mut ans = 0;
    let mut imos = vec![0; n+1];
    for i in (0..n).rev() {
        imos[i] = imos[i+1] + a[i].abs();
    }

    let mut sum = 0;
    let mut head = 0;
    while head < n {
        if a[head] < 0 {
            let mut to = head + 1;
            let mut part_minus = a[head];
            let mut hoge = a[head];
            while to < n && a[to] >= 1 {
                part_minus -= a[to];
                hoge += a[to];
                to += 1;
            }
            ans = max(ans, sum + part_minus + imos[to]);
            sum += hoge;
            head = to;
        } else {
            sum += a[head];
            head += 1;
        }
    }
    ans = max(ans, sum);

    println!("{}", ans);
}