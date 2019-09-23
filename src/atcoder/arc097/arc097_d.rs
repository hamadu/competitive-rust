// https://atcoder.jp/contests/arc097/tasks/arc097_d
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

    ($iter:expr, [ next / $t:tt ]) => {
        {
            let len = read_value!($iter, usize);
            (0..len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
        }
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
macro_rules! dvec {
    ($t:expr ; $len:expr) => {
        vec![$t; $len]
    };

    ($t:expr ; $len:expr, $($rest:expr),*) => {
        vec![dvec!($t; $($rest),*); $len]
    };
}

#[allow(unused_macros)]
macro_rules! ifv {
    ($t:expr, $a:expr, $b: expr) => {
        if $t { $a } else { $b }
    }
}

#[allow(unused_macros)]
macro_rules! fill {
    ($t:expr, $v:expr) => {
        for i in 0..$t.len() {
            $t[i] = $v;
        }
    };
}

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

fn diameter(root: usize, graph: &Vec<Vec<usize>>, available: &Vec<bool>, white: &Vec<bool>) -> usize {
    let (tidx, _) = dfs(root, None, graph, available, white);
    let (_, ret) = dfs(tidx, None, graph, available, white);
    ret
}

fn dfs(now: usize, par: Option<usize>, graph: &Vec<Vec<usize>>, available: &Vec<bool>, white: &Vec<bool>) -> (usize, usize) {
    let mut best = (0, 0);
    if white[now] {
        best = (now, 1);
    }

    for &to in &graph[now] {
        if Some(to) == par || !available[to] {
            continue;
        }
        let (res, num) = dfs(to, Some(now), graph, available, white);
        let tnum = num + if white[now] { 1 } else { 0 };
        if best.1 < tnum {
            best = (res, tnum);
        }
    }
    best
}

fn main() {
    input! {
        n: usize,
        edges: [(usize1, usize1); n-1],
        colors: chars
    };
    if n == 1 {
        println!("{}", ifv!(colors[0] == 'B', 0, 1));
        return;
    }

    let mut graph = vec![vec![]; n];
    let mut deg = vec![0; n];
    for (u, v) in edges {
        graph[u].push(v);
        graph[v].push(u);
        deg[u] += 1;
        deg[v] += 1;
    }

    let mut que = VecDeque::new();
    let mut available = vec![true; n];
    for i in 0..n {
        if colors[i] == 'B' && graph[i].len() == 1 {
            available[i] = false;
            que.push_back(i);
        }
    }
    while let Some(v) = que.pop_front() {
        for &to in &graph[v] {
            if available[to] {
                deg[to] -= 1;
                if deg[to] == 1 && colors[to] == 'B' {
                    available[to] = false;
                    que.push_back(to);
                }
            }
        }
    }
    let mut ac = 0;
    for i in 0..n {
        if available[i] {
            ac += 1;
        }
    }
    if ac <= 1 {
        println!("{}", if ac == 0 { 0 } else { 1 });
        return;
    }

    let mut cost = 2*(ac-1);
    let mut root = 0;
    let mut is_white = vec![false; n];
    for i in 0..n {
        if available[i] {
            is_white[i] = (deg[i] % 2 == 0) ^ (colors[i] == 'B');
            if is_white[i] {
                cost += 1;
            }
            root = i;
        }
    }
    // debug!(available, is_white, root);
    // debug!(cost);
    // debug!(diameter(root, &graph, &available, &is_white));

    println!("{}", cost - diameter(root, &graph, &available, &is_white) * 2);
}
