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

const MOD: i64 = 1e9 as i64 + 7;

fn powmod(a: i64, p: usize, m: i64) -> i64 {
    let mut ret = 1i64;
    let mut aa = a;
    let mut pp = p;
    while pp >= 1 {
        if pp & 1 == 1 {
            ret *= aa;
            ret %= m;
        }
        aa = aa * aa % m;
        pp >>= 1;
    }
    ret
}

fn inv(a: i64, m: i64) -> i64 {
    powmod(a, m as usize - 2, m)
}

struct Combination {
    fact: Vec<i64>,
    invfact: Vec<i64>,
    modulo: i64
}

impl Combination {
    fn new(n: usize, modulo: i64) -> Self {
        let mut fact: Vec<i64> = vec![0; n];
        let mut invfact: Vec<i64> = vec![0; n];
        fact[0] = 1;
        for i in 1..n {
            fact[i] = fact[i-1] * i as i64 % modulo;
        }
        invfact[n-1] = inv(fact[n-1], modulo);
        for i in (0..n-1).rev() {
            invfact[i] = (invfact[i+1] * (i+1) as i64) % modulo;
        }

        Combination { fact: fact, invfact: invfact, modulo: modulo }
    }

    fn combination(&self, n: usize, k: usize) -> i64 {
        if n < k {
            return 0;
        }
        self.fact[n] * self.invfact[n-k] % self.modulo * self.invfact[k] % self.modulo
    }
}

// ===

struct SCCEdge {
    n: usize,
    ord: Vec<i32>,
    low: Vec<i32>,
    graph: Vec<Vec<usize>>,
    edge_components: Vec<Vec<(usize, usize)>>,
    oi: i32,
    root: Vec<bool>,
    tmp_edges: VecDeque<(usize, usize)>
}

impl SCCEdge {
    fn new(graph: Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        SCCEdge {
            n: n,
            ord: vec![-1; n],
            low: vec![n as i32; n],
            root: vec![false; n],
            oi: 0,
            edge_components: vec!(),
            tmp_edges: VecDeque::new(),
            graph: graph
        }
    }

    fn build(&mut self) {
        for i in 0..self.n {
            if self.ord[i] == -1 {
                self.root[i] = true;
                let n = self.n;
                self.dfs(i, n);
            }
        }
    }

    fn dfs(&mut self, now: usize, par: usize) {
        if self.ord[now] != -1 {
            return;
        }
        self.ord[now] = self.oi;
        self.low[now] = self.oi+1;
        self.oi += 1;

        let ni = self.graph[now].len();
        for i in 0..ni {
            let to = self.graph[now][i];
            if to == par {
                continue
            }
            if self.ord[to] < self.ord[now] {
                self.tmp_edges.push_front((now, to));
            }

            if self.ord[to] == -1 {
                self.dfs(to, now);
                self.low[now] = min(self.low[now], self.low[to]);
                if self.low[to] >= self.ord[now] {
                    let mut edges = vec![];
                    while let Some(edge) = self.tmp_edges.pop_front() {
                        edges.push(edge);
                        let (e0, e1) = (min(edge.0, edge.1), max(edge.0, edge.1));
                        let (u0, u1) = (min(now, to), max(now, to));
                        if e0 == u0 && e1 == u1 {
                            break
                        }
                    }
                    self.edge_components.push(edges);
                }
            } else {
                self.low[now] = min(self.low[now], self.ord[to]);
            }
        }
    }
}

fn main() {
    let (n, m, k): (usize, usize, i64) = read();
    let edges: Vec<(usize, usize)> = readn::<(usize, usize)>(m).into_iter().map(|x| (x.0-1, x.1-1)).collect();

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for &(u, v) in &edges {
        graph[u].push(v);
        graph[v].push(u)
    }

    let mut scc = SCCEdge::new(graph);
    scc.build();

    let comb = Combination::new(2000, MOD);
    let mut ans: i64 = 1;
    for edges in scc.edge_components {
        let en = edges.len();
        if en == 1 {
            ans *= k;
            ans %= MOD;
        } else {
            let mut vset = HashSet::new();
            for &(u, v) in &edges {
                vset.insert(u);
                vset.insert(v);
            }
            if vset.len() == en {
                ans *= paint_cycle(en, k);
                ans %= MOD;
            } else {
                let k = k as usize;
                ans *= comb.combination(en + k - 1, k - 1);
                ans %= MOD;
            }
        }
    }
    println!("{}", ans);
}

fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b)
    }
}

fn paint_cycle(n: usize, k: i64) -> i64 {
    let mut total = 0;

    for i in 0..n {
        let g = gcd(n, i);
        total += powmod(k, g, MOD);
    }
    total %= MOD;
    total * inv(n as i64, MOD) % MOD
}
