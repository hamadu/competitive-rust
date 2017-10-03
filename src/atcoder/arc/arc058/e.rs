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
parse_tuple!(A, B, C, D);

// ===

const MOD: i32 = 1000000007;

fn main() {
    let (n, x, y, z): (usize, usize, usize, usize) = read();

    let mut dp: Vec<Vec<i32>> = vec![vec![0; 1<<18]; 2];
    dp[0][0] = 1;

    let mask = (1<<17)-1;
    let cmask = 1<<17;
    let okmask = (1<<(x+y+z-1))|(1<<(y+z-1))|(1<<(z-1));

    for i in 0..n {
        let fr = i % 2;
        let to = 1 - fr;
        for ptn in 0..(1<<18) {
            dp[to][ptn] = 0;
        }

        for ptn in 0..(1<<18) {
            let base = dp[fr][ptn];
            if base == 0 {
                continue
            }
            for x in 1..11 {
                let tptn = ((ptn<<x|1<<(x-1))&mask);
                let ok = ptn&cmask >= 1 || tptn&okmask == okmask;
                let tptn = tptn | if ok { cmask } else { 0 };

                dp[to][tptn] += base;
                if dp[to][tptn] >= MOD {
                    dp[to][tptn] -= MOD;
                }
            }
        }
    }

    let mut sum: i64 = 0;
    for ptn in 0..(1<<18) {
        if ptn & cmask >= 1 {
            sum += dp[n%2][ptn] as i64;
        }
    }
    println!("{}", sum % MOD as i64);
}