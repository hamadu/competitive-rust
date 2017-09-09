#![allow(unused_imports, unused_variables, dead_code)]
use std::io::*;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;

pub trait InputValue {
    fn parse(s: &str) -> Self;
}

pub fn read<T: InputValue>() -> T {
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    T::parse(&buf.trim())
}

pub fn readnc<T: InputValue>() -> Vec<T> {
    let mut vec = vec![];
    let line: String = read();
    for token in line.split_whitespace() {
        vec.push(T::parse(token));
    }
    vec
}

pub fn readn<T: InputValue>(n: usize) -> Vec<T> {
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

fn main() {
    let (h, w): (usize, usize) = read();
    let table: Vec<String> = readn(h);
    let table: Vec<Vec<char>> = table.into_iter().map(|a| a.chars().collect()).collect();

    let mut diff: Vec<Vec<bool>> = vec![vec![false; w-1]; h];
    for i in 0..h {
        for j in 0..w-1 {
            diff[i][j] = table[i][j] == table[i][j+1];
        }
    }


    let mut ans = max(w, h);

    let mut height: Vec<usize> = vec![0; w-1];
    for i in 0..h {
        for j in 0..w-1 {
            let f = diff[i][j];
            if height[j] == i {
                while height[j] < h && diff[height[j]][j] == f {
                    height[j] += 1;
                }
            }
        }

        let mut h: Vec<usize> = vec![0; w-1];
        for j in 0..w-1 {
            h[j] = height[j] - i;
        }
        ans = max(ans, doit(h));
    }

    println!("{}", ans);
}

fn doit(a: Vec<usize>) -> usize {
    let n = a.len();
    let mut right: Vec<usize> = vec![0; n];
    {
        let mut stk: Vec<usize> = vec![];
        for i in (0..n).rev() {
            while stk.len() >= 1 && a[i] <= a[stk[stk.len()-1]] {
                stk.pop();
            }
            right[i] = if stk.len() >= 1 { stk[stk.len()-1] } else { n };
            stk.push(i);
        }
    }

    let mut left: Vec<usize> = vec![0; n];
    {
        let mut stk: Vec<usize> = vec![];
        for i in 0..n {
            while stk.len() >= 1 && a[i] <= a[stk[stk.len()-1]] {
                stk.pop();
            }
            left[i] = if stk.len() >= 1 { stk[stk.len()-1]+1 } else { 0 };
            stk.push(i);
        }
    }

    let mut ans = 0;
    for i in 0..n {
        ans = max(ans, (right[i] - left[i] + 1) * a[i]);
    }
    ans
}