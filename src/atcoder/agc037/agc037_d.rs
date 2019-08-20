// https://atcoder.jp/contests/agc037/tasks/agc037_d
//
#![allow(unused_imports)]
use std::io::*;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused_macros)]
macro_rules! dvec {
    ($t:expr ; $len:expr) => {
        vec![$t; $len]
    };

    ($t:expr ; $len:expr, $($rest:expr),*) => {
        vec![dvec!($t; $($rest),*); $len]
    };
}

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

const INF: usize = 1e9 as usize;

fn print_table(a: &Vec<Vec<usize>>) {
    for al in a {
        println!("{}", al.iter().map(|x| (x+1).to_string()).collect::<Vec<_>>().join(" "));
    }
}

struct XorShift {
    x: usize,
    y: usize,
    z: usize,
    w: usize
}

impl XorShift {
    fn new() -> XorShift {
        XorShift {
            x: 123456789,
            y: 362436069,
            z: 521288629,
            w: 88675123
        }
    }

    fn rotate(&mut self) {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = (self.w ^ (self.w >> 19)) ^ (t ^ (t >> 8));
    }

    fn next_int(&mut self, n: usize) -> usize {
        self.rotate();
        let r = (self.w as i32) % (n as i32);
        if r < 0 {
            (r + n as i32) as usize
        } else {
            r as usize
        }
    }
}


fn gen(n: usize, m: usize) -> Vec<Vec<usize>> {
    let mut x = XorShift::new();
    let mut aw = vec![0; n*m];
    for i in 0..n {
        for j in 0..m {
            aw[i*m+j] = i*m+j;
        }
    }
    for i in 1..n*m {
        let w = x.next_int(i);
        let t = aw[i];
        aw[i] = aw[w];
        aw[w] = t;
    }

    let mut a = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            a[i][j] = aw[i*m+j];
        }
    }
    a
}

// ===

const MAX_FLOW: i32 = 100000000;

#[derive(Clone, Copy)]
struct Edge {
    from: usize,
    to: usize,
    cap: i32,
    maxcap: i32,
    label: usize,
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

    fn edge(&mut self, from: usize, to: usize, cap: i32, label: usize) {
        let e = Edge { from: from, to: to, cap: cap, maxcap: cap, label: label };
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

// ====

fn main() {
    input! {
        n: usize, m: usize,
        a: [[usize1; m]; n]
    };

    // let mut a = a;
    // let mut a = gen(n, m);
    // print_table(&a);

    let mut pmap = dvec!(0; n*m, 2);
    for i in 0..n {
        for j in 0..m {
            pmap[a[i][j]][0] = i;
            pmap[a[i][j]][1] = j;
        }
    }

    let mut b = dvec!(INF; n, m);
    for wj in 0..m {
        let mut mat = FlowGraph::new(n*2+2);
        let source = n+n;
        let sink = n+n+1;
        for j in 0..n {
            mat.edge(source, j, 1, INF);
            mat.edge(n+j, sink, 1, INF);
        }
        for z in 0..n*m {
            let fi = pmap[z][0];
            if fi == INF {
                continue;
            }
            mat.edge(z / m, n + fi, 1, z);
        }

        let flow = mat.max_flow(source, sink);
        for fi in 0..n {
            for &eid in &mat.graph[fi] {
                let edge = mat.edges[eid];
                if edge.cap == 0 && n <= edge.to && edge.to < 2*n {
                    // used
                    b[edge.to-n][wj] = edge.label;
                    pmap[edge.label][0] = INF;
                    break;
                }
            }
        }
    }

    let mut c = dvec!(INF; n, m);
    for j in 0..m {
        for i in 0..n {
            c[b[i][j]/m][j] = b[i][j];
        }
    }
    print_table(&b);
    print_table(&c);
}
