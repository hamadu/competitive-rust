// https://atcoder.jp/contests/agc037/tasks/agc037_e
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
macro_rules! dvec {
    ($t:expr ; $len:expr) => {
        vec![$t; $len]
    };

    ($t:expr ; $len:expr, $($rest:expr),*) => {
        vec![dvec!($t; $($rest),*); $len]
    };
}

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

// ===

/// Suffix Array. Build: O(N(logN)^2), where N = s.len()
/// sa[i]  := i-th lexicographically smaller suffix of original string.
///           We can obtain actual string by s[sa[i]..].
///           In this implementation, we'll consider empty string "" is
///           one of the suffix of the given string. Therefore,
///           sa[0] is always N and |sa| = N+1.
/// lcp[i] := length of common prefix of s[sa[i]..] and s[sa[i+1]..].
///           |lcp| = N.
struct SuffixArray {
    s: Vec<char>,
    sa: Vec<usize>,
    lcp: Vec<usize>,
}

impl SuffixArray {
    fn compare(i: usize, j: usize, len: usize, rank: &Vec<i32>) -> Ordering {
        if rank[i] != rank[j] {
            rank[i].cmp(&rank[j])
        } else {
            let ri = *rank.get(i+len).unwrap_or(&-1);
            let rj = *rank.get(j+len).unwrap_or(&-1);
            ri.cmp(&rj)
        }
    }

    fn new(s: Vec<char>) -> SuffixArray {
        let n = s.len();
        let mut sa = vec![0; n+1];
        let mut rank = vec![0; n+1];
        for i in 0..n+1 {
            sa[i] = i;
            rank[i] = if i < n { s[i] as i32 } else { -1 };
        }

        let mut tmp = vec![0; n+1];
        let mut step = 1;
        while step <= n {
            sa.sort_by(|&a, &b| SuffixArray::compare(a, b, step, &rank));
            tmp[sa[0]] = 0;
            for i in 1..n+1 {
                let a = sa[i-1];
                let b = sa[i];
                tmp[b] = tmp[a] + if SuffixArray::compare(a, b, step, &rank) == Ordering::Less { 1 } else { 0 };
            }
            for i in 0..n+1 {
                rank[i] = tmp[i];
            }
            step *= 2;
        }

        let mut lcp = vec![0; n];
        for i in 0..n+1 {
            rank[sa[i]] = i as i32;
        }
        let mut h = 0;
        for i in 0..n {
            let idx = rank[i] as usize-1;
            let j = sa[idx];
            if h > 0 {
                h -= 1;
            }
            while j + h < n && i + h < n {
                if s[j+h] != s[i+h] {
                    break;
                }
                h += 1;
            }
            lcp[idx] = h;
        }
        SuffixArray { s: s, sa: sa, lcp: lcp }
    }
}

impl std::fmt::Debug for SuffixArray {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let n = self.s.len();
        let vtos = |c: &Vec<char>, i, j| c[i..j].iter().map(|&c| c.to_string()).collect::<Vec<_>>().join("");
        writeln!(f, "SA {{");
        writeln!(f, "  original: |{}| = {}",  vtos(&self.s, 0, n), n);
        writeln!(f, "----  {:>2}: ", 0);
        for i in 1..n+1 {
            let sa = self.sa[i];
            writeln!(f, "^{:>2}   {:>2}: {}", self.lcp[i-1], i, vtos(&self.s, sa, n));
        }
        writeln!(f, "}}")
    }
}


fn find_best(s: Vec<char>) -> Vec<char> {
    let n = s.len();
    let half = n / 2;
    let su = SuffixArray::new(s.clone());
    let mut best = vec!['~'; half];
    for i in 1..n+1 {
        let w = su.sa[i];
        if n-w >= half {
            best.copy_from_slice(&s[w..w+half]);
            break;
        }
    }
    best
}

fn main() {
    input! {
        n: usize, k: usize,
        s: chars
    };


    let mut s = s;
    for _ in 0..min(k-1, 12) {
        let mut best = vec!['~'; n];
        let mut ss = s.clone();
        ss.reverse();
        s.append(&mut ss);
        s.reverse();

        s = find_best(s);
        s.reverse();

        // debug!(s);
    }

    let mut ss = s.clone();
    ss.reverse();
    s.append(&mut ss);
    let ans = find_best(s);
    println!("{}", ans.into_iter().map(|c| c.to_string()).collect::<Vec<_>>().join(""));
}
