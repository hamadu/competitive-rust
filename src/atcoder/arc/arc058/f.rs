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
    let (n, k): (usize, usize) = read();
    let s: Vec<String> = readn(n);
    let s: Vec<Vec<u8>> = s.into_iter().map(|s| s.into_bytes()).collect();

    let mut available: Vec<Vec<i32>> = vec![vec![0; k+1]; n+1];
    available[n][k] = 1;
    for i in (0..n).rev() {
        let l = s[i].len();
        for j in 0..k+1 {
            if available[i+1][j] == 1 || (j + l <= k && available[i+1][j+l] == 1) {
                available[i][j] = 1;
            }
        }
    }
    assert!(available[0][0] == 1);

    let mut prev: Vec<u8> = vec![];

    let mut cut: Vec<bool> = vec![false; k+1];
    cut[0] = true;


    for i in 0..n {
        let l = s[i].len();
        let mut largest = k + 1;
        for j in 0..k+1 {
            if available[i+1][j] != 1 {
                continue
            }

            let left = j >= l && cut[j-l] && available[i][j-l] == 1;
            if left {
                let suffix = is_larger(&prev, &s[i], largest, j-l, k);
                if suffix >= 0 {
                    largest = j-l;
                }
            }
        }

        if largest <= k {
            let mut next: Vec<u8> = vec![];
            for i in 0..largest {
                next.push(prev[i]);
            }
            for &c in &s[i] {
                next.push(c);
            }

            let mut next_cut: Vec<bool> = vec![false; k+1];
            for x in 0..min(next.len(), prev.len())+1 {
                next_cut[x] = cut[x];
                if x >= min(next.len(), prev.len()) || prev[x] != next[x] {
                    break;
                }
            }
            next_cut[0] = true;
            next_cut[largest] = true;
            next_cut[next.len()] = true;

            for pi in 0..largest {
                if !cut[pi] || available[i+1][pi+s[i].len()] == 0 {
                    continue
                }

                let mut sub: Vec<u8> = vec![];
                for l in 0..pi {
                    sub.push(prev[l]);
                }
                for &c in &s[i] {
                    sub.push(c);
                }
                if pi + s[i].len() <= next.len() && comp(&sub, &next) <= 0 {
                    next_cut[pi+s[i].len()] = true;
                }
            }


//            println!("{}, {}, {}", i, next_cut[1], s[i][0] as char);
//            for &c in &next {
//                print!("{}", c as char);
//            }
//            println!();

            prev = next;
            cut = next_cut;
        }
    }

    for c in prev {
        print!("{}", c as char);
    }
    println!();
}

// compare
//   A = s[0..j) and
//   B = s[0..j-t.len()) + t
// returns true if B < A.
fn is_larger(s: &Vec<u8>, t: &Vec<u8>, t1: usize, t2: usize, k: usize) -> i32 {
    let mut wo2: Vec<u8> = vec![];
    for i in 0..t2 {
        wo2.push(s[i]);
    }
    for &c in t {
        wo2.push(c);
    }

    let mut wo: Vec<u8> = vec![];

    if t1 == k + 1 {
        for &c in s {
            wo.push(c);
        }
    } else {
        for i in 0..t1 {
            wo.push(s[i]);
        }
        for &c in t {
            wo.push(c);
        }
    }
    comp(&wo, &wo2)
}

fn comp(s: &Vec<u8>, t: &Vec<u8>) -> i32 {
    for i in 0..min(s.len(), t.len()) {
        if s[i] != t[i] {
            return if t[i] < s[i] { 1 } else { -1 };
        }
    }
    return 0;
}