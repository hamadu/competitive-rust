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

const MAX_FLOW: i32 = 100000000;

#[derive(Clone, Copy)]
struct Edge {
    from: usize,
    to: usize,
    cap: i32,
    maxcap: i32
}

impl Edge {
    fn goto(&self, from: usize) -> usize {
        self.from + self.to - from
    }

    fn cap(&self, from: usize) -> i32 {
        if self.from == from {
            self.cap
        } else {
            self.maxcap - self.cap
        }
    }
}

struct FlowGraph {
    n: usize,
    graph: Vec<Vec<usize>>,
    edges: Vec<Edge>,
    level: Vec<i32>,
    itr: Vec<usize>
}

impl FlowGraph {
    fn new(n: usize) -> Self {
        let graph = vec![vec![]; n];
        let level = vec![0; n];
        let itr = vec![0; n];
        FlowGraph { n: n, graph: graph, edges: vec![], level: level, itr: itr }
    }

    fn edge(&mut self, from: usize, to: usize, cap: i32) {
        let e = Edge { from: from, to: to, cap: cap, maxcap: cap };
        let l = self.edges.len();
        self.graph[from].push(l);
        self.graph[to].push(l);
        self.edges.push(e);
    }

    fn dfs(&mut self, now: usize, to: usize, flow: i32) -> i32 {
        if now == to {
            return flow;
        }
        for i in self.itr[now]..self.graph[now].len() {
            self.itr[now] = i;
            let eidx = self.graph[now][i];

            let e = self.edges[eidx];
            let next = e.goto(now);
            let ecap = e.cap(now);
            if ecap > 0 && self.level[now] < self.level[next] {
                let d = self.dfs(next, to, min(flow, ecap));
                if d >= 1 {
                    let e = &mut self.edges[eidx];
                    if e.from == now {
                        e.cap -= d;
                    } else {
                        e.cap += d;
                    }
                    return d;
                }
            }

        }
        return 0;
    }

    fn bfs(&mut self, from: usize) {
        for i in 0..self.n {
            self.level[i] = -1;
        }
        self.level[from] = 0;
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(from);
        while let Some(idx) = q.pop_front() {
            for &eidx in &self.graph[idx] {
                let e = &self.edges[eidx];
                let to = e.goto(idx);
                if e.cap(idx) >= 1 && self.level[to] == -1 {
                    self.level[to] = self.level[idx] + 1;
                    q.push_back(to);
                }
            }
        }
    }

    fn max_flow(&mut self, from: usize, to: usize) -> i32 {
        let mut flow = 0;
        loop {
            self.bfs(from);
            if self.level[to] < 0 {
                return flow;
            }
            for i in 0..self.n {
                self.itr[i] = 0;
            }
            loop {
                let f = self.dfs(from, to, MAX_FLOW);
                if f <= 0 {
                    break;
                }
                flow += f;
            }
        }
    }
}


// ===

fn main() {
    let (h, w): (usize, usize) = read();

    let a: Vec<String> = readn(h);
    let a: Vec<Vec<u8>> = a.into_iter().map(|s| s.into_bytes()).collect();

    let source = h + w;
    let sink = source + 1;
    let mut maxflow = FlowGraph::new(h+w+2);
    for i in 0..h {
        for j in 0..w {
            let c = a[i][j] as char;
            if c == 'o' {
                maxflow.edge(i, h+j, 1);
                maxflow.edge(h+j, i, 1);
            } else if c == 'S' {
                maxflow.edge(source, i, 100000);
                maxflow.edge(source, h+j, 100000);
            } else if c == 'T' {
                maxflow.edge(i, sink, 100000);
                maxflow.edge(h+j, sink, 100000);
            }
        }
    }

    let ret = maxflow.max_flow(source, sink);
    let ans = if ret >= 100000 { -1 } else { ret };

    println!("{}", ans);
}