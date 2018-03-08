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

fn dfs(now: usize, par: usize, depth: usize, graph: &Vec<Vec<usize>>) -> usize {
    if depth == 0 {
        return 1;
    }
    let mut ret = 1;
    for &to in &graph[now] {
        if to == par {
            continue
        }
        ret += dfs(to, now, depth-1, graph);
    }
    ret
}

fn main() {
    let (n, k): (usize, usize) = read();
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..n-1 {
        let (a, b): (usize, usize) = read();
        graph[a-1].push(b-1);
        graph[b-1].push(a-1);
    }

    let mut best = 0;
    if k % 2 == 1 {
        let half = (k-1)/2;
        for i in 0..n {
            for &j in &graph[i] {
                if i < j {
                    best = max(best, dfs(i, j, half, &graph) + dfs(j, i, half, &graph))
                }
            }
        }
    } else {
        let half = k/2;
        for i in 0..n {
            best = max(best, dfs(i, n, half, &graph));
        }
    }

    println!("{}", n-best);
}