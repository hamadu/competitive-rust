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
    let n: usize = read();
    let edges: Vec<(usize, usize)> = readn(n-1);

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..n-1 {
        let a = edges[i].0-1;
        let b = edges[i].1-1;
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut par: Vec<usize> = vec![0; n];
    let mut depth: Vec<usize> = vec![0; n];
    let mut count: Vec<usize> = vec![0; n];
    dfs(0, 0, n, &graph, &mut par, &mut depth, &mut count);

    let cnt = depth[n-1] + 1;
    let mut head = n-1;
    for i in 0..(cnt/2-1) {
        head = par[head];
    }
    let fennec = n - count[head];
    let snuke = count[head];
    if fennec > snuke {
        println!("Fennec");
    } else {
        println!("Snuke");
    }
}

fn dfs(now: usize, dep: usize, par: usize, graph: &Vec<Vec<usize>>, parent: &mut Vec<usize>, depth: &mut Vec<usize>, count: &mut Vec<usize>) {
    parent[now] = par;
    depth[now] = dep;
    count[now] = 1;
    for &to in &graph[now] {
        if to == par {
            continue;
        }
        dfs(to, dep+1, now, graph, parent, depth, count);
        count[now] += count[to];
    }
}
