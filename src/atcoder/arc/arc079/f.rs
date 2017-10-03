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

const INF: usize = 100000000;

fn main() {
    let n: usize = read();
    let go: Vec<usize> = readnc();
    let go: Vec<usize> = go.into_iter().map(|p| p-1).collect();

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    let mut pa: Vec<usize> = vec![0; n];
    let mut deg: Vec<usize> = vec![0; n];
    for i in 0..n {
        deg[go[i]] += 1;
        pa[i] = go[i];
        graph[go[i]].push(i);
    }

    let mut is_loop: Vec<bool> = vec![false; n];
    let mut head = 0;
    for i in 0..n {
        head = pa[head];
    }
    for i in 0..n {
        is_loop[head] = true;
        head = pa[head];
    }

    let mut value: Vec<usize> = vec![INF; n];
    let mut que: VecDeque<usize> = VecDeque::new();
    for i in 0..n {
        if deg[i] == 0 {
            que.push_back(i);
        }
    }

    while let Some(v) = que.pop_front() {
        value[v] = compute(v, &value, &graph);
        deg[pa[v]] -= 1;
        if deg[pa[v]] == 0 {
            que.push_back(pa[v]);
        }
    }

    let v = head;
    let mut is_ok = false;

    let mut tried = 0;
    let mut set: HashSet<usize> = HashSet::new();
    for &t in &graph[v] {
        set.insert(value[t]);
    }
    for vv in 0..n {
        if set.contains(&vv) {
            continue;
        }
        tried += 1;
        if tried >= 3 {
            break;
        }
        for i in 0..n {
            if is_loop[i] {
                value[i] = INF;
            }
        }
        value[v] = vv;

        let mut head = v;
        loop {
            head = pa[head];
            if head == v {
                break;
            }
            value[head] = compute(head, &value, &graph);
        }
        is_ok |= verify(&value, &graph);
    }
    println!("{}", if is_ok { "POSSIBLE" } else { "IMPOSSIBLE" });
}

fn verify(value: &Vec<usize>, graph: &Vec<Vec<usize>>) -> bool {
    let n = graph.len();
    for i in 0..n {
        for &to in &graph[i] {
            if value[i] == value[to] {
                return false;
            }
        }
        if value[i] != compute(i, value, graph) {
            return false;
        }
    }
    true
}

fn compute(v: usize, value: &Vec<usize>, graph: &Vec<Vec<usize>>) -> usize {
    let n = graph.len();
    let mut set: HashSet<usize> = HashSet::new();
    for &t in &graph[v] {
        set.insert(value[t]);
    }
    let mut i = 0;
    while i < n {
        if !set.contains(&i) {
            return i;
        }
        i += 1;
    }
    panic!("ohno")
}