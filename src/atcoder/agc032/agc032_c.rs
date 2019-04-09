// https://atcoder.jp/contests/agc032/tasks/agc032_c
//
#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use std::fmt::*;
use std::io::*;
use std::str::*;

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

type Edge = (usize, usize, bool);
type Graph = Vec<Vec<Edge>>;

fn main() {
    input! {
        n: usize, m: usize,
        edges: [(usize1, usize1); m]
    };

    let mut graph: Graph = vec![vec![]; n];
    for e in edges {
        let e0 = graph[e.0].len();
        let e1 = graph[e.1].len();
        graph[e.0].push((e.1, e1, true));
        graph[e.1].push((e.0, e0, true));
    }

    if isok(&mut graph) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn isok(graph: &mut Graph) -> bool {
    let n = graph.len();
    let mut max_deg = 0;
    let mut deg = vec![0; n];
    let mut deg4 = 0;
    for i in 0..n {
        let v = &graph[i];
        if v.len() % 2 == 1 {
            return false;
        }
        deg[i] = v.len();
        max_deg = max(max_deg, v.len());
        if deg[i] == 4 {
            deg4 += 1;
        }
    }
    if max_deg >= 6 {
        return true;
    }
    if max_deg == 2 {
        return false;
    }
    if deg4 >= 3 {
        return true;
    }
    if deg4 == 1 {
        return false;
    }

    let mut lpcnt = 0;
    loop {
        let mut touch = vec![false; n];
        let mut deq: VecDeque<usize> = VecDeque::new();
        let mut initv = n;
        for i in 0..n {
            if deg[i] >= 1 {
                initv = i;
                deq.push_back(i);
                break;
            }
        }
        if initv == n {
            break;
        }
        let mut first = true;
        while deq.len() >= 1 {
            let v = *deq.back().unwrap();
            if !first && v == initv {
                lpcnt += 1;
                break;
            }
            first = false;
            if touch[v] {
                deq.pop_back(); // pop back v once

                lpcnt += 1;
                while deq.len() >= 1 {
                    let front = *deq.back().unwrap();
                    assert!(touch[front]);
                    touch[front] = false;
                    if front == v {
                        break;
                    }
                    deq.pop_back();
                }
                continue;
            }

            let mut u = n;
            let mut uidx = n;
            for &e in &graph[v] {
                if e.2 {
                    u = e.0;
                    uidx = e.1;
                    break;
                }
            }
            assert!(u != n);
            let vidx = graph[u][uidx].1;

            assert!(u != n);
            assert!(u != v);
            assert!(graph[u][uidx].0 == v);
            assert!(graph[v][vidx].0 == u);
            assert!(graph[u][uidx].2);
            assert!(graph[v][vidx].2);

            graph[u][uidx].2 = false;
            graph[v][vidx].2 = false;
            deg[u] -= 1;
            deg[v] -= 1;
            deq.push_back(u);
            touch[v] = true;
        }
        if lpcnt >= 3 {
            return true;
        }
    }
    return false;
}
