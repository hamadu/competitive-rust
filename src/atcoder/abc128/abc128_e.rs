// https://atcoder.jp/contests/abc128/tasks/abc128_e
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

fn main() {
    input! {
        n: usize, q: usize,
        roadworks: [(i64, i64, i64); n],
        queries: [i64; q]
    };

    let mut events = vec![];
    for i in 0..q {
        events.push((queries[i], 2, 0));
    }
    for i in 0..n {
        let f = roadworks[i].0-roadworks[i].2;
        let t = roadworks[i].1-roadworks[i].2;
        events.push((f, 1, roadworks[i].2));
        events.push((t, 0, roadworks[i].2));
    }
    events.sort();
    // debug!(events);

    let mut que = BinaryHeap::new();
    let mut rem = HashSet::new();
    for (at, qtype, position) in events {
        match qtype {
            0 => {
                rem.insert(position);
            },
            1 => {
                que.push(-position);
                rem.remove(&position);
            },
            2 => {
                let mut has = false;
                while let Some(w) = que.pop() {
                    let w = -w;
                    if rem.contains(&w) {
                        continue;
                    } else {
                        has = true;
                        println!("{}", w);
                        que.push(-w);
                        break;
                    }
                }
                if !has {
                    println!("-1");
                }
            },
            _ => unreachable!("invalid query type")
        }
    }
}
