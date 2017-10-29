#![allow(unused_imports, unused_variables, dead_code)]
use std::io::*;
use std::io::Write;
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

fn question(a: usize, b: usize) -> bool {
    println!("? {} {}", a, b);
    stdout().flush();
    let s: String = read();
    return s == "Y";
}

fn main() {
    let (mut a, mut b): (usize, usize) = read();
    if a <= b {
        println!("Impossible");
        return
    }

    let mut chain: Vec<usize> = vec![];

    let n = a + b;
    let mut seen: Vec<bool> = vec![false; n];

    loop {
        let mut ask = n;
        let mut target = n;
        if chain.len() == 0 {
            for i in 0..n {
                if !seen[i] {
                    ask = i;
                    seen[i] = true;
                    break;
                }
            }
        } else {
            ask = chain[chain.len()-1];
        }
        for i in 0..n {
            if !seen[i] {
                seen[i] = true;
                target = i;
                break
            }
        }
        if target == n {
            chain.push(ask);
            break;
        }

        let ok = question(ask, target);
        if ok {
            if chain.len() == 0 {
                chain.push(ask);
            }
            chain.push(target);
        } else {
            if chain.len() >= 1 {
                let l = chain.len()-1;
                chain.remove(l);
            }
            b -= 1;
        }

        if chain.len() > b {
            break;
        }
    }

    let honest = chain[chain.len()-1];
    let mut ans = vec![false; n];
    for i in 0..n {
        if question(honest, i) {
            ans[i] = true;
        }
    }

    print!("! ");
    for i in 0..n {
        if ans[i] {
            print!("1");
        } else {
            print!("0");
        }
    }
    println!();
}