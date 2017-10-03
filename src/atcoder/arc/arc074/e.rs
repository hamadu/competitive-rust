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

const MOD: i32 = 1000000007;

fn main() {
    let (n, m): (usize, usize) = read();
    let q: Vec<(usize, usize, usize)> = readn(m);
    let mut vec: Vec<Vec<(usize, usize)>> = vec![vec![]; n+1];
    for (l, r, c) in q {
        vec[r].push((l, c));
    }

    let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; n+1]; n+1]; n+1];
    dp[0][0][0] = 1;

    for x in 0..n {
        for p in 0..n {
            for q in 0..n {
                let base = dp[x][p][q];
                if base == 0 {
                    continue
                }

                let r = x + 1;
                for (tp, tq) in vec![(p, q), (x, q), (p, x)] {
                    let mut ok = true;
                    for &(l, c) in &vec[x+1] {
                        if color(tp, tq, l, r) != c {
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        dp[x+1][tp][tq] += base;
                        if dp[x+1][tp][tq] >= MOD {
                            dp[x+1][tp][tq] -= MOD;
                        }
                    }
                }
            }
        }
    }

    let mut sum: i64 = 0;
    for p in 0..n {
        for q in 0..n {
            sum += dp[n][p][q] as i64;
        }
    }
    sum %= MOD as i64;

    println!("{}", sum);
}

fn color(a: usize, b: usize, l: usize, r: usize) -> usize {
    let mut ret = 1;
    if l <= a && a <= r {
        ret += 1;
    }
    if l <= b && b <= r {
        ret += 1;
    }
    ret
}