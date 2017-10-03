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

const MOD: i64 = 1000000007;

fn powmod(a: i64, p: i64, m: i64) -> i64 {
    let mut ret = 1i64;
    let mut aa = a;
    let mut pp = p;
    while pp >= 1 {
        if pp & 1 == 1 {
            ret *= aa;
            ret %= m;
        }
        aa = aa * aa % m;
        pp >>= 1;
    }
    ret
}


fn main() {
    let (n, c): (usize, usize) = read();
    let a: Vec<usize> = readnc();
    let b: Vec<usize> = readnc();

    let mut ranges: Vec<(usize, usize)> = vec![];
    for i in 0..n {
        ranges.push((a[i], b[i]));
    }

    let mut pow_k: Vec<Vec<i64>> = vec![vec![0; 401]; 401];
    for k in 0..401 {
        let mut sum: i64 = 0;
        for i in 1..401 {
            sum += powmod(i, k, MOD);
            if sum >= MOD {
                sum -= MOD;
            }
            pow_k[k as usize][i as usize] = sum;
        }
    }

    let mut memo: Vec<Vec<i64>> = vec![vec![-1; c+1]; n+1];

    println!("{}", dfs(0, c, &ranges, &pow_k, &mut memo));
}

fn dfs(i: usize, left: usize, range: &Vec<(usize, usize)>, pow_k: &Vec<Vec<i64>>, memo: &mut Vec<Vec<i64>>) -> i64 {
    if memo[i][left] != -1 {
        return memo[i][left]
    }
    if i == range.len() {
        let ret = if left == 0 { 1 } else { 0 };
        memo[i][left] = ret;
        return ret;
    }

    let rng = range[i];
    let mut sum = 0;
    for u in 0..left+1 {
        let mut p = pow_k[u][rng.1] - pow_k[u][rng.0-1] + MOD;
        if p >= MOD {
            p -= MOD;
        }
        sum += p * dfs(i+1, left-u, range, pow_k, memo) % MOD;
        if sum >= MOD {
            sum -= MOD;
        }
    }
    memo[i][left] = sum;
    sum
}