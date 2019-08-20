// https://atcoder.jp/contests/agc034/tasks/agc034_d
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
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

const INF: Cost = 1e12 as Cost;

type Capacity = i64;
type Cost = i64;
type Graph = Vec<Vec<usize>>;

struct Edge {
    to: usize,
    rev: usize,
    cap: Capacity,
    cost: Cost,
}

struct MinCostFlow {
    n: usize,
    graph: Graph,
    edges: Vec<Edge>,
}

type State = (i64, usize);

impl MinCostFlow {
    fn new(n: usize) -> Self {
        MinCostFlow {
            n: n,
            graph: vec![vec![]; n],
            edges: vec![],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, cap: Capacity, cost: Cost) {
        let eidx = self.edges.len();
        self.edges.push(Edge { to: v, rev: eidx+1, cap: cap, cost: cost });
        self.edges.push(Edge { to: u, rev: eidx, cap: 0, cost: -cost });
        self.graph[u].push(eidx);
        self.graph[v].push(eidx+1);
    }

    fn min_cost_flow(&mut self, from: usize, to: usize, flow: Capacity) -> Cost {
        let n = self.graph.len();
        let mut remain_flow = flow;
        let mut ans = 0;
        let mut from_v = vec![0; n];

        while remain_flow > 0 {
            let mut dist = vec![INF; n];
            let mut upd = vec![false; n];
            dist[from] = 0;
            upd[from] = true;

            let mut q = VecDeque::new();
            q.push_back(from);
            while q.len() >= 1 {
                let v = q.pop_front().unwrap();
                upd[v] = false;
                for &eid in &self.graph[v] {
                    let edge = &self.edges[eid];
                    if edge.cap == 0 {
                        continue;
                    }
                    let t = edge.to;
                    let val = dist[v] + edge.cost;
                    if dist[t] > val {
                        dist[t] = val;
                        from_v[t] = eid;
                        if !upd[t] {
                            upd[t] = true;
                            q.push_back(t);
                        }
                    }
                }
            }
            if dist[to] == INF {
                return ans;
            }

            {
                let mut min_cap = remain_flow;
                let mut tt = to;
                while tt != from {
                    let edge = &self.edges[from_v[tt]];
                    min_cap = min(min_cap, edge.cap);
                    tt = self.edges[from_v[tt]^1].to;
                }
                remain_flow -= min_cap;
                ans += min_cap * dist[to];
                let mut tt = to;
                while tt != from {
                    let eid = from_v[tt];
                    let reid = eid ^ 1;
                    self.edges[eid].cap -= min_cap;
                    self.edges[reid].cap += min_cap;
                    tt = self.edges[reid].to;
                }
            }
        }
        ans
    }
}

fn main() {
    input! {
        n: usize,
        reds: [(i64, i64, i64); n],
        blues: [(i64, i64, i64); n],
    };

    // [0, n): red
    // [n, 2n): blue
    // [2n, 2n+4): center
    // 2n+4: source
    // 2n+5: sink
    let mut min_cost_flow = MinCostFlow::new(2*n+2+4);
    let offset = 2000000000;
    let source = 2*n+4;
    let sink = 2*n+5;

    let mut total_flow = 0;
    for i in 0..n {
        total_flow += reds[i].2;
        min_cost_flow.add_edge(source, i, reds[i].2 as Capacity, 0);
        for j in 0..2 {
            for k in 0..2 {
                let tj = j as i64 * 2 - 1;
                let tk = k as i64 * 2 - 1;
                min_cost_flow.add_edge(i, 2*n+j*2+k, reds[i].2 as Capacity,      (reds[i].0 * tj) +  (reds[i].1 * tk) + offset);
                min_cost_flow.add_edge(2*n+j*2+k, n+i, blues[i].2 as Capacity, -(blues[i].0 * tj) - (blues[i].1 * tk) + offset);
            }
        }
        min_cost_flow.add_edge(n+i, sink, blues[i].2 as Capacity, 0);
    }

    println!("{}", total_flow * offset * 2 - min_cost_flow.min_cost_flow(source, sink, total_flow));
}
