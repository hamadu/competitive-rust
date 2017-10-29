#![allow(unused_imports, unused_variables, dead_code)]
use std::io::*;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;
use std::io::stdout;
use std::io::Write;

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

const A: i64 = 1;

fn query(n: i64) -> bool {
    println!("? {}", n);
    stdout().flush();
    read::<String>() == "Y"
//     if (A >= n) ^ (A.to_string() >= n.to_string()) {
//         false
//     } else {
//         true
//     }
}

fn main() {
    let mut q = 1;
    let mut qq = 0;
    while q <= 1e11 as i64 {
        if !query(q) {
            qq = q / 10;
            break
        }
        q *= 10;
    }

    if qq == 0 {
        q = 9;
        let mut ans = 1;
        while true {
            if query(q) {
               break;
            }
            ans *= 10;
            q *= 10;
            q += 9;
        }
        println!("! {}", ans);
    } else {
        let mut from = 0;
        let mut to = 9;

        let mut ans = 0;
        while qq >= 1 {
            while to - from > 1 {
                let med = (to + from) / 2;
                let tq = (ans + qq * med + qq - 1) * 10 + 9;
                if query(tq) {
                    to = med;
                } else {
                    from = med;
                }
            }
            ans += qq * to;
            qq /= 10;
            from = -1;
            to = 9;
        }
        println!("! {}", ans);
    }
}