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

fn print_with_spaces(a: &Vec<usize>) {
    let n = a.len();
    for i in 0..n {
        if i >= 1 {
            print!(" ");
        }
        print!("{}", a[i]);
    }
    println!();
}

fn main() {
    let (n, m): (usize, usize) = read();
    let am: Vec<usize> = readnc();

    let mut odd: Vec<usize> = vec![];
    let mut even: Vec<usize> = vec![];
    for &ai in &am {
        if ai % 2 == 1 {
            odd.push(ai);
        } else {
            even.push(ai);
        }
    }

    if odd.len() >= 3 {
        println!("Impossible");
    } else {
        let mut a: Vec<usize> = vec![];
        let mut b: Vec<usize> = vec![];
        if odd.len() >= 1 {
            a.push(odd[0]);
        }
        for i in even {
            a.push(i);
        }
        if odd.len() == 2 {
            a.push(odd[1]);
        }

        if a.len() == 1 {
            if n == 1 {
                b.push(1);
            } else {
                b.push(a[0]-1);
                b.push(1);
            }
        } else {
            if a[0] >= 2 {
                b.push(a[0]-1);
            }
            for i in 1..a.len() {
                if i == a.len()-1 {
                    b.push(a[i]+1);
                } else {
                    b.push(a[i]);
                }
            }
        }

        print_with_spaces(&a);
        println!("{}", b.len());
        print_with_spaces(&b);
    }

}