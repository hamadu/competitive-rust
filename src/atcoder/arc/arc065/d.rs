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

// ===

fn main() {
    let (n, k, l): (usize, usize, usize) = read();

    let mut walk = UnionFind::new(n);
    let mut rail = UnionFind::new(n);

    for i in 0..k {
        let (a, b): (usize, usize) = read();
        walk.unite(a-1, b-1);
    }
    for i in 0..l {
        let (a, b): (usize, usize) = read();
        rail.unite(a-1, b-1);
    }

    let mut map = HashMap::new();
    for i in 0..n {
        let id = (walk.find(i), rail.find(i));
        let next = *map.get(&id).unwrap_or(&0);
        map.insert(id, next+1);
    }

    for i in 0..n {
        let id = (walk.find(i), rail.find(i));
        if i >= 1 {
            print!(" ");
        }
        print!("{}", map.get(&id).unwrap());
    }
    println!();
}