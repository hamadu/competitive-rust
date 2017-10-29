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

struct BucketArray {
    n: usize,
    bn: usize,
    bucket_size: usize,
    data: Vec<i32>,
    bucket: Vec<i32>,
}

impl BucketArray {
    fn new(n: usize) -> Self {
        let bucket_size = max(50, (n as f64).sqrt() as usize);
        let bn = (n + bucket_size - 1) / bucket_size;

        BucketArray {
            n: n,
            bn: bn,
            bucket_size: bucket_size,
            data: vec![0; n],
            bucket: vec![0; bn]
        }
    }

    fn less(&self, x: usize, default: usize) -> usize {
        let bi = x / self.bucket_size;
        for xi in (bi*self.bucket_size..x).rev() {
            if self.data[xi] == 1 {
                return xi;
            }
        }
        for bi in (0..bi).rev() {
            if self.bucket[bi] >= 1 {
                let from = bi * self.bucket_size;
                for xi in (from..min(from+self.bucket_size, self.n)).rev() {
                    if self.data[xi] >= 1 {
                        return xi;
                    }
                }
            }
        }
        default
    }

    fn more(&self, x: usize, default: usize) -> usize {
        let bi = x / self.bucket_size;
        for bi in bi..self.bn {
            if self.bucket[bi] >= 1 {
                let from = bi * self.bucket_size;
                for xi in max(x+1, from)..min(from+self.bucket_size, self.n) {
                    if self.data[xi] >= 1 {
                        return xi;
                    }
                }
            }
        }
        default
    }

    fn add(&mut self, x: usize) {
        self.bucket[x / self.bucket_size] += 1;
        self.data[x] += 1;
    }

    fn remove(&mut self, x: usize) {
        self.bucket[x / self.bucket_size] -= 1;
        self.data[x] -= 1;
    }
}

fn main() {
    let (n, m): (usize, usize) = read();
    let dist: Vec<i64> = readnc();

    let mut data: Vec<Vec<i64>> = vec![vec![0; n+3]; n+3];

    let mut values: Vec<Vec<i32>> = vec![vec![0; n]; m];
    for i in 0..n {
        let c: Vec<i32> = readnc();
        for j in 0..m {
            values[j][i] = c[j];
        }
    }

    for i in 0..m {
        let mut c: Vec<(i32, usize)> = values[i].clone().into_iter().zip(1..n+1).collect();
        c.sort();
        c.reverse();

        let mut bset = BucketArray::new(n+2);
        bset.add(0);
        bset.add(n+1);

        for ci in c {
            let v = ci.0 as i64;
            let idx = ci.1;
            let fx = bset.less(idx, 0)+1;
            let tx = idx+1;

            let fy = idx;
            let ty = bset.more(idx, n+1);

            data[fx][fy] += v;
            data[tx][fy] -= v;
            data[fx][ty] -= v;
            data[tx][ty] += v;

            bset.add(idx);
        }
    }

    for i in 0..n+2 {
        for j in 1..n+2 {
            data[i][j] += data[i][j-1];
        }
    }
    for j in 0..n+2 {
        for i in 1..n+2 {
            data[i][j] += data[i-1][j];
        }
    }

    let mut dx = vec![0; n];
    for i in 1..n {
        dx[i] += dx[i-1] + dist[i-1];
    }

    // 1 to n : 1 to n
    let mut ans = 0;
    for from in 1..n+1 {
        for to in from..n+1 {
            ans = max(ans, data[from][to] - (dx[to-1] - dx[from-1]));
        }
    }
    println!("{}", ans);
}