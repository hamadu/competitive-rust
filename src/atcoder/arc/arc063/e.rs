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

const INF: i32 = 1000000000;

fn main() {
    let n: usize = read();
    let edges: Vec<(usize, usize)> = readn(n-1);
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for &(u, v) in &edges {
        graph[u-1].push(v-1);
        graph[v-1].push(u-1)
    }

    let k: usize = read();
    let mut dp: Vec<i32> = vec![INF; n];
    let mut q = BinaryHeap::new();
    for i in 0..k {
        let (v, p): (usize, i32) = read();
        dp[v-1] = p;
        q.push((-p, v-1));
    }

    while let Some((mp, now)) = q.pop() {
        let value = -mp;
        let next = value + 1;

        for &to in &graph[now] {
            if dp[to] == INF {
                dp[to] = next;
                q.push((-next, to));
            }
        }
    }

    let mut is_ok = true;
    for u in 0..n {
        for &v in &graph[u] {
            if (dp[u]-dp[v]).abs() != 1 {
                is_ok = false;
            }
        }
    }

    if is_ok {
        println!("Yes");
        for i in 0..n {
            println!("{}", dp[i]);
        }
    } else {
        println!("No");
    }
}