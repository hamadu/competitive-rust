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

fn error_function(a: &Vec<usize>) -> Vec<usize> {
    let n = a.len();
    let mut err: Vec<usize> = vec![0; n+1];

    for i in 2..n+1 {
        let mut now = err[i-1];
        while now > 0 && a[i-1] != a[now] {
            now = err[now];
        }
        if a[i-1] == a[now] {
            err[i] = now + 1;
        } else {
            err[i] = 0;
        }
    }
    err
}

fn compute(s: &Vec<usize>, c: usize, l: usize) -> Vec<usize> {
    let n = s.len();
    let mut ret = vec![0; 26];
    let mut par = vec![0; 26];
    if n % c == 0 {
        for i in 0..n {
            par[s[i]] += 1;
        }
        let cn = l/n;
        for i in 0..26 {
            ret[i] = par[i] * cn;
        }
        for i in 0..l%n {
            ret[s[i]] += 1;
        }
    } else {
        let mut len = vec![0; 120];
        let mut vec = vec![vec![0; 26]; 120];
        for i in 0..n {
            vec[0][s[i]] += 1;
            vec[1][s[i]] += 1;
        }
        for i in 0..c {
            vec[1][s[i]] += 1;
        }

        len[0] = n;
        len[1] = n + c;
        for i in 2..len.len() {
            len[i] = len[i-1] + len[i-2];
            for j in 0..26 {
                vec[i][j] = vec[i-1][j] + vec[i-2][j];
            }
            if len[i] >= l {
                return count(i, l, &len, &vec, &s)
            }
        }
    }
    ret
}

fn count(level: usize, head: usize, len: &Vec<usize>, vec: &Vec<Vec<usize>>, s: &Vec<usize>) -> Vec<usize> {
    let mut ret = vec![0; 26];
    if level <= 1 {
        let n = s.len();
        for i in 0..head {
            ret[s[i%n]] += 1;
        }
        ret
    } else {
        if head <= len[level-1] {
            count(level-1, head, len, vec, s)
        } else {
            let x = count(level-2, head - len[level-1], len, vec, s);
            for i in 0..26 {
                ret[i] += vec[level-1][i];
                ret[i] += x[i];
            }
            ret
        }
    }
}

fn main() {
    let s: String = read();
    let s = s.into_bytes();
    let mut s: Vec<usize> = s.into_iter().map(|x| (x - 'a' as u8) as usize).collect();
    let n = s.len();
    let s = s.split_off(n / 2);

    let (l, r): (usize, usize) = read();
    let n = s.len();
    let err = error_function(&s);
    let cycle = n - err[err.len()-1];

    let ll = compute(&s, cycle, l-1);
    let rr = compute(&s, cycle, r);

    for i in 0..26 {
        if i >= 1 {
            print!(" ");
        }
        print!("{}", rr[i] - ll[i]);
    }
    println!();
}