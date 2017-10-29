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

const INF: usize = 100000000;
const MAX: usize = 5010;

fn main() {
    let n: usize = read();
    let mut p: Vec<usize> = readnc();
    p.insert(0, n);
    for i in 1..n {
        p[i] -= 1;
    }

    let x: Vec<usize> = readnc();

    let mut children: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 1..n {
        children[p[i]].push(i);
    }


    let mut best: Vec<usize> = vec![INF; n];
    let mut dp: Vec<Vec<usize>> = vec![vec![INF; MAX]; 2];

    let mut isok = true;
    for i in (0..n).rev() {
        let cn = children[i].len();
        for i in 0..MAX {
            dp[0][i] = INF;
            dp[1][i] = INF;
        }

        dp[0][0] = 0;
        for ci in 0..cn {
            let a = x[children[i][ci]];
            let b = best[children[i][ci]];

            let fi = ci % 2;
            let ti = 1 - fi;
            for i in 0..MAX {
                dp[ti][i] = INF;
            }
            for i in 0..MAX {
                if dp[fi][i] == INF {
                    continue
                }
                let base = dp[fi][i];
                let ia = i + a;
                if ia < MAX {
                    dp[ti][ia] = min(dp[ti][ia], base + b);
                }
                let ib = i + b;
                if ib < MAX {
                    dp[ti][ib] = min(dp[ti][ib], base + a);
                }
            }
        }

        let wi = cn % 2;
        let mut best_op = INF;
        for l in 0..x[i]+1 {
            best_op = min(best_op, dp[wi][l]);
        }

        if best_op == INF {
            isok = false;
            break
        }
        best[i] = best_op;
    }

    if isok {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}