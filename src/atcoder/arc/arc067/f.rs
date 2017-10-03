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

const MAX: i32 = 1000000;
const MIN: i32 = -100;

fn main() {
    let (n, m): (usize, usize) = read();
    let distances: Vec<i64> = readnc();
    let mut dimos: Vec<i64> = vec![];
    dimos.push(0);

    let mut last: i64 = 0;
    for di in distances {
        last += di;
        dimos.push(last);
    }

    let mut value: Vec<Vec<i32>> = vec![vec![0; n]; m];
    for i in 0..n {
        let v: Vec<i32> = readnc();
        for j in 0..m {
            value[j][i] = v[j];
        }
    }

    let mut table: Vec<Vec<i64>> = vec![];
    for i in 0..5005 {
        table.push(vec![0; i+5]);
    }

    let mut arr: Vec<bool> = vec![false; n];
    let mut barr: Vec<bool> = vec![false; 75];

    for j in 0..m {
        let val = &value[j];
        let mut row: Vec<(&i32, usize)> = val.into_iter().zip((0..n).into_iter()).collect();
        row.sort_by(|a, b| a.0.cmp(b.0));
        row.reverse();

        for i in 0..n {
            arr[i] = false;
        }
        for i in 0..75 {
            barr[i] = false;
        }

        for (&val, idx) in row {
            let ty: i32 = larger((idx+1), &arr, &barr) as i32;
            let fx: i32 = (smaller(idx, &arr, &barr) + 1) as i32;
            let ty = ty as usize;
            let fx = fx as usize;

            let v = val as i64;
            table[idx][fx] += v;
            table[idx][idx+1] -= v;
            table[ty][fx] -= v;
            table[ty][idx+1] += v;

            arr[idx] = true;
            barr[idx / 75] = true;
        }
    }

    for i in 0..table.len() {
        for j in 1..table[i].len() {
            table[i][j] += table[i][j-1];
        }
    }
    for j in 0..table.len() {
        for i in 1..table.len() {
            if j < table[i].len() && j < table[i-1].len() {
                table[i][j] += table[i-1][j];
            }
        }
    }

    let mut ans: i64 = 0;
    for i in 0..n {
        for j in 0..(i+1) {
            let d: i64 = (dimos[i] - dimos[j]) as i64;
            ans = max(ans, table[i][j] - d);
        }
    }

    println!("{}", ans);
}


fn smaller(idx: usize, arr: &Vec<bool>, barr: &Vec<bool>) -> i32 {
    let v = idx / 75;
    for b in (0..(v+1)).rev() {
        if barr[b] {
            let from = min(idx, (b + 1) * 75);
            let to = b * 75;
            for i in (to..from).rev() {
                if arr[i] {
                    return i as i32;
                }
            }

        }
    }
    -1
}

fn larger(idx: usize, arr: &Vec<bool>, barr: &Vec<bool>) -> i32 {
    let v = idx / 75;
    for b in v..(barr.len()) {
        if barr[b] {
            let from = max(idx, b * 75);
            let to = min(arr.len(), (b + 1) * 75);
            for i in from..to {
                if arr[i] {
                    return i as i32;
                }
            }

        }
    }
    arr.len() as i32
}