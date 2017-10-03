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
parse_tuple!(A, B, C, D);

#[derive(PartialEq,PartialOrd)]
struct Of64(f64);

impl Eq for Of64 {}

impl Ord for Of64 {
    fn cmp(&self, other: &Of64) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


// ===

struct Point {
    x: i64,
    y: i64,
    r: i64
}

impl Point {
    fn distance(&self, to: &Point) -> f64 {
        let dx = (self.x-to.x).abs();
        let dy = (self.y-to.y).abs();
        let dist: f64 = ((dx*dx+dy*dy) as f64).sqrt();
        f64::max(0.0, dist - (self.r + to.r) as f64)
    }
}



const INF: f64 = 1e18;

fn main() {
    let (xs, ys, xt, yt): (i64, i64, i64, i64) = read();
    let n = read();
    let barrier: Vec<(i64, i64, i64)> = readn(n);

    let mut points: Vec<Point> = vec![];
    points.push(Point { x: xs, y: ys, r: 0 });
    points.push(Point { x: xt, y: yt, r: 0 });
    for &b in &barrier {
        points.push(Point { x: b.0, y: b.1, r: b.2 });
    }

    let n = points.len();
    let mut graph: Vec<Vec<f64>> = vec![vec![INF; n]; n];
    for i in 0..n {
        for j in i+1..n {
            let d = points[i].distance(&points[j]);
            graph[i][j] = d;
            graph[j][i] = d;
        }
        graph[i][i] = 0.0;
    }

    let mut q = BinaryHeap::new();
    let mut dp: Vec<f64> = vec![INF; n];
    dp[0] = 0.0;

    q.push((Of64(0.0), 0));

    while let Some((Of64(md), now)) = q.pop() {
        let d = -md;
        if dp[now] < d {
            continue
        }
        for to in 0..n {
            let td = d + graph[now][to];
            if dp[to] > td {
                dp[to] = td;
                q.push((Of64(-td), to));
            }
        }
    }

    println!("{}", dp[1]);
}