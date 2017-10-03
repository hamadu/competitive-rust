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

const INF: i32 = 1000000000;

fn main() {
    let (n, m, r): (usize, usize, usize) = read();
    let rr: Vec<usize> = readnc();
    let rr: Vec<usize> = rr.into_iter().map(|r| r-1).collect();

    let mut graph: Vec<Vec<i32>> = vec![vec![INF; n]; n];
    for _ in 0..m {
        let (a, b, c): (usize, usize, i32) = read();
        graph[a-1][b-1] = c;
        graph[b-1][a-1] = c;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                graph[i][j] = min(graph[i][j], graph[i][k] + graph[k][j]);
            }
        }
    }

    let mut dp = vec![vec![INF; r]; 1<<r];
    for ri in 0..r {
        dp[1<<ri][ri] = 0;
    }

    for i in 0..1<<r {
        for now in 0..r {
            if dp[i][now] == INF {
                continue
            }
            for ri in 0..r {
                if i & (1 << ri) >= 1 {
                    continue
                }
                let ti = i | (1 << ri);
                dp[ti][ri] = min(dp[ti][ri], dp[i][now] + graph[rr[now]][rr[ri]]);
            }
        }
    }

    let mut ans = INF;
    for now in 0..r {
        ans = min(ans, dp[(1<<r)-1][now]);
    }
    println!("{}", ans);
}