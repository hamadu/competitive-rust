#![allow(unused_imports, unused_variables, dead_code)]
use std::io::*;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;

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

// ===


#[derive(Debug)]
struct Range {
    from: usize,
    to: usize,
    queries: Vec<(usize, usize, usize, usize)>
}

fn main() {
    let (n, m): (usize, usize) = read();
    let mut edges: Vec<(usize, usize, usize)> = vec![];

    let mut uf = UnionFind::new(n);
    for i in 0..m {
        let (u, v): (usize, usize) = read();
        let u = u-1;
        let v = v-1;
        if uf.same(u, v) {
            continue
        }
        edges.push((u, v, i))
    }

    let q: usize = read();
    let mut xyz = vec![];
    for i in 0..q {
        let (x, y, z): (usize, usize, usize) = read();
        xyz.push((x-1, y-1, z, i));
    }

    let en = edges.len();
    let mut ranges = vec![Range { from: 0, to: en, queries: xyz }];

    let mut ans = vec![0; q];

    loop {
        let mut uf = UnionFind::new(n);
        let mut next_ranges = vec![];
        let n = ranges.len();
        for i in 0..n {
            let mut next_left = vec![];
            let mut next_right = vec![];

            let fr = ranges[i].from;
            let to = ranges[i].to;
            let queries = &ranges[i].queries;
            // let Range { from: fr, to: to, queries: queries } = ranges[i];
            let med = (fr + to) / 2;
            for ei in fr..med {
                let (u, v, _) = edges[ei];
                uf.unite(u, v);
            }

            // hantei
            for &(x, y, z, idx) in queries {
                let sum;
                if uf.same(x, y) {
                    sum = uf.count(x);
                } else {
                    sum = uf.count(x) + uf.count(y);
                }
                if sum >= z {
                    next_left.push((x, y, z, idx));
                } else {
                    next_right.push((x, y, z, idx));
                }
            }

            for ei in med..to {
                let (u, v, _) = edges[ei];
                uf.unite(u, v);
            }

            if to - fr == 1 {
                for &(x, y, z, idx) in queries {
                    ans[idx] = edges[fr].2+1;
                }
            } else {
                if med - fr >= 1 {
                    next_ranges.push(Range { from: fr, to: med, queries: next_left })
                }
                if to - med >= 1 {
                    next_ranges.push(Range { from: med, to: to, queries: next_right })
                }
            }
        }

        if next_ranges.len() == 0 {
            break
        }
        // println!("{:?}", next_ranges);
        ranges = next_ranges;
    }

    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    for i in 0..q {
        writeln!(out, "{}", ans[i]);
    }
}