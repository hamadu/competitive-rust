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

fn readvec<T: InputValue>() -> Vec<T> {
    let mut vec = vec![];
    let line: String = read();
    for token in line.split_whitespace() {
        vec.push(T::parse(token));
    }
    vec
}

fn readlines<T: InputValue>(n: usize) -> Vec<T> {
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
                ($($t::parse(tokens.next().unwrap())),*)
            }
        }
    }
}
parse_tuple!(A, B);
parse_tuple!(A, B, C);

// ===

const WALL: u8 = '#' as u8;
const ROOM: u8 = '.' as u8;
const INF: usize = 10000000;

const DX: [i32; 4] = [0, -1, 0, 1];
const DY: [i32; 4] = [-1, 0, 1, 0];

fn main() {
    let (h, w): (usize, usize) = read();
    let map: Vec<Vec<u8>> = readlines(h).into_iter().map(|s: String| s.into_bytes()).collect();

    let mut dp = vec![vec![INF; w]; h];
    let mut que = VecDeque::new();
    que.push_back(0);
    que.push_back(0);
    dp[0][0] = 1;
    while que.len() >= 2 {
        let y = que.pop_front().unwrap();
        let x = que.pop_front().unwrap();
        let base = dp[y][x];

        for d in 0..4 {
            let ty = y as i32 + DY[d];
            let tx = x as i32 + DX[d];
            if ty < 0 || tx < 0 || ty >= h as i32 || tx >= w as i32 {
                continue;
            }
            let ty = ty as usize;
            let tx = tx as usize;
            if map[ty][tx] == WALL {
                continue;
            }
            if dp[ty as usize][tx as usize] == INF {
                dp[ty as usize][tx as usize] = base+1;
                que.push_back(ty);
                que.push_back(tx);
            }
        }
    }

    let mut total = 0;
    for i in 0..h {
        for j in 0..w {
            if map[i][j] == ROOM {
                total += 1;
            }
        }
    }
    if dp[h-1][w-1] == INF {
        println!("-1")
    } else {
        println!("{}", total - dp[h-1][w-1])
    }
}