// https://atcoder.jp/contests/agc025/tasks/agc025_d
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

fn paint(idx: usize, graph: &Vec<Vec<usize>>, color: i32, result: &mut Vec<i32>) {
    if result[idx] != 0 {
        return;
    }
    result[idx] = color;
    for &to in &graph[idx] {
        paint(to, graph, -color, result);
    }
}

fn generate(n: usize, d: i64) -> Vec<Vec<i32>> {
    let mut sqrt_table = vec![-1; 2*n*n];
    for i in 0..2*n+1 {
        let i2 = i * i;
        if i2 < sqrt_table.len() {
            sqrt_table[i2] = i as i64;
        }
    }


    let mut ds: Vec<(i64, i64)> = vec![];
    for x in 0..n+1 {
        let xi = x as i64;
        let y2 = d - xi * xi;
        if y2 < 0 {
            continue;
        }
        if y2 >= sqrt_table.len() as i64 {
            continue;
        }
        let yi = sqrt_table[y2 as usize];
        if yi == -1 {
            continue;
        }
        ds.push((xi, yi));
    }

    let mut graph = vec![vec![]; n*n];
    let ni = n as i64;
    for i in 0..ni {
        for j in 0..ni {
            let fid = (i * ni + j) as usize;
            for &(x, y) in &ds {
               for dx in 0..2 {
                    for dy in 0..2 {
                        let ix = i + x * (dx * 2 - 1);
                        let iy = j + y * (dy * 2 - 1);
                        if ix < 0 || iy < 0 || ix >= ni || iy >= ni {
                            continue;
                        }
                        let tid = (ix * ni + iy) as usize;
                        if fid < tid {
                            graph[fid].push(tid);
                            graph[tid].push(fid);
                        }
                    }
                }
            }
        }
    }

    let mut result = vec![0; n*n];
    let mut field = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let id = i*n+j;
            if result[id] == 0 {
                paint(id, &graph, 1, &mut result);
            }
            field[i][j] = result[id];
        }
    }
    // println!("{:?}", &field);

    field
}

fn main() {
    input! {
        n: usize, d1: i64, d2: i64
    };

    let graph1 = generate(2*n, d1);
    let graph2 = generate(2*n, d2);

    let mut table = vec![vec![0; 2*n]; 2*n];
    for i in 0..2*n {
        for j in 0..2*n {
            let a = if graph1[i][j] == 1 { 0 } else { 1 };
            let b = if graph2[i][j] == 1 { 0 } else { 2 };
            table[i][j] = a+b;
        }
    }


    let mut cnt = vec![0; 4];
    let mut wx = 0;
    for i in 0..2*n {
        for j in 0..2*n {
            cnt[table[i][j]] += 1;
            if cnt[table[i][j]] > cnt[wx] {
                wx = table[i][j];
            }
        }
    }

    let mut cnt = 0;
    for i in 0..2*n {
        for j in 0..2*n {
            if table[i][j] == wx {
                cnt += 1;
                if cnt <= n*n {
                    println!("{} {}", i, j);
                }
            }
        }
    }
}
