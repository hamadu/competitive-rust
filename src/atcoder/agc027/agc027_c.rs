// https://atcoder.jp/contests/agc027/tasks/agc027_c
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


fn has_cycle(graph: Vec<Vec<usize>>) -> bool {
    let n = graph.len();
    let mut indeg = vec![0; n];
    for i in 0..n {
        for &j in graph[i].iter() {
            indeg[j] += 1;
        }
    }

    let mut deq = VecDeque::new();
    let mut cnt = 0;
    for i in 0..n {
        if indeg[i] == 0 {
            deq.push_back(i);
            cnt += 1;
        }
    }

    while deq.len() >= 1 {
        let u = deq.pop_front().unwrap();
        for &j in graph[u].iter() {
            indeg[j] -= 1;
            if indeg[j] == 0 {
                deq.push_back(j);
                cnt += 1;
            }
        }
    }
    return cnt != n;
}

fn main() {
    input! {
        n: usize, m: usize,
        s: chars,
        edges: [(usize1, usize1); m]
    };


    let mut snum = vec![0; n];
    for i in 0..n {
        if s[i] == 'B' {
            snum[i] = 1;
        }
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in edges.iter() {
        if u == v {
            graph[u].push(v);
        } else {
            graph[u].push(v);
            graph[v].push(u);
        }
    }

    let mut dgraph = vec![vec![]; 2*n];
    for i in 0..n {
        for &t in graph[i].iter() {
            // i*2(R,R) -> t*2(R,B)
            // i*2+1(B,R) -> t*2(R,B)
            let tt = t*2 + (snum[i] ^ snum[t] ^ 1);
            if snum[t] == snum[i] {
                dgraph[i*2].push(tt);
            }
            if snum[t] != snum[i] {
                dgraph[i*2+1].push(tt);
            }
        }
    }

    if has_cycle(dgraph) {
        println!("Yes");
    } else {
        println!("No");
    }
}
