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

struct UnionFind {
    n: usize,
    rank: Vec<usize>,
    parent: Vec<usize>,
    count: Vec<usize>
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut p = vec![0; n];
        for i in 0..n {
            p[i] = i;
        }
        UnionFind { n: n, rank: vec![0; n], parent: p, count: vec![1; n] }
    }

    fn find(&mut self, a: usize) -> usize {
        if self.parent[a] == a {
            return a;
        }
        let par = self.parent[a];
        self.parent[a] = self.find(par);
        self.parent[a]
    }

    fn unite(&mut self, a: usize, b: usize) {
        let a = self.find(a);
        let b = self.find(b);
        if a == b {
            return;
        }
        let total = self.count[a] + self.count[b];
        self.count[b] = total;
        self.count[a] = total;

        let ra = self.rank[a];
        let rb = self.rank[b];
        if ra < rb {
            self.parent[a] = b;
        } else {
            self.parent[b] = a;
            if ra == rb {
                self.rank[a] += 1;
            }
        }
    }

    fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn count(&mut self, a: usize) -> usize {
        let a = self.find(a);
        self.count[a]
    }
}

fn powmod(a: i64, p: i64, m: i64) -> i64 {
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
    powmod(a, m-2, m)
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

const MOD: i64 = 1e9 as i64 + 7;

// ===

struct Solver {
    n: usize,
    cnt: usize,
    graph: Vec<Vec<usize>>,
    visited: Vec<bool>,
}

impl Solver {
    fn new(g: Vec<Vec<usize>>) -> Self {
        let n = g.len();
        Solver { n: n, cnt: 0, graph: g, visited: vec![false; n] }
    }

    fn solve(&mut self) -> i64 {
        let (lp, lv, mut pp) = self.find_loop();


        let mut total = 0;
        let ln = lv.len();
        for i in 0..ln {
            pp[lv[i]] = pp[lv[(i+1)%ln]];
        }
        total += self.doit(&pp);

        for i in 0..ln {
            pp[lv[i]] = pp[lv[(i-1+ln)%ln]];
        }
        total += self.doit(&pp);

        total % MOD
    }

    fn doit(&mut self, pp: &Vec<usize>) -> i64 {
        1
    }


    fn find_loop(&mut self) -> (Vec<bool>, Vec<usize>, Vec<usize>) {
        let n = self.n;
        let mut isloop = vec![true; n];
        let mut deg = vec![0; n];
        let mut q = vec![];
        for i in 0..n {
            deg[i] = self.graph[i].len();
            if deg[i] == 1 {
                isloop[i] = false;
                q.push(i);
            }
        }
        let mut head = 0;
        while head < q.len() {
            let leaf = q[head];
            for &to in &self.graph[leaf] {
                if isloop[to] {
                    deg[to] -= 1;
                    if deg[to] == 1 {
                        isloop[to] = false;
                        q.push(to);
                    }
                }
            }
            head += 1;
        }

        let mut lv = vec![];
        let mut pp = vec![0; n];
        for i in 0..n {
            if isloop[i] {
                self.dfs0(i, n, &isloop, &mut lv, &mut pp);
                break
            }
        }
        (isloop, lv, pp)
    }

    fn dfs0(&mut self, now: usize, par: usize, isloop: &Vec<bool>, lv: &mut Vec<usize>, pp: &mut Vec<usize>) {
        if self.visited[now] {
            return
        }
        self.visited[now] = true;
        pp[now] = par;

        if isloop[now] {
            lv.push(now);
        }
        let n = self.graph[now].len();
        for ti in 0..n {
            let to = self.graph[now][ti];
            if to != par {
                self.dfs0(to, now, isloop, lv, pp);
            }
        }
    }
}

// ===

fn main() {
    let n: usize = read();
    let balls: Vec<(usize, usize)> = readn(2*n);

    let mut comb = Combination::new(200000, MOD);

    let mut uf = UnionFind::new(2*n);
    for i in 0..2*n {
        let (x, y) = balls[i];
        uf.unite(x-1, n+y-1);
    }

    let mut group = vec![vec![]; 2*n];
    for i in 0..2*n {
        group[uf.find(i)].push(i);
    }

    let mut xmap = vec![0; n+1];
    let mut ymap = vec![0; n+1];

    let mut ans = 1;
    let mut total = 2*n;
    for i in 0..2*n {
        let size = group[i].len();
        if size == 0 {
            continue
        }
        let mut xs = vec![];
        let mut ys = vec![];
        for &bi in &group[i] {
            let (x, y) = balls[bi];
            xs.push(x);
            ys.push(y);
        }
        xs.sort();
        xs.dedup();
        ys.sort();
        ys.dedup();
        for i in 0..xs.len() {
            xmap[xs[i]] = i;
        }
        for i in 0..ys.len() {
            ymap[ys[i]] = i;
        }


        let n = xs.len() + ys.len();
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        let mut deg = 0;
        for &bi in &group[i] {
            let (x, y) = balls[bi];
            let xi = xmap[x];
            let yi = ymap[y];

            graph[xi].push(xs.len()+yi);
            graph[xs.len()+yi].push(xi);
            deg += 1;
        }

        if deg != graph.len() {
            ans = 0;
            break
        }

        let mut solver = Solver::new(graph);
        ans *= solver.solve();
        ans %= MOD;
        ans *= comb.combination(total, size);
        ans %= MOD;
        total -= size;
    }

    println!("{}", ans);
}