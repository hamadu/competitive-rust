// https://atcoder.jp/contests/dwacon5th-final-open/tasks/dwacon5th_final_b
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



#[derive(Debug)]
struct BinaryTrie {
    nodes: Vec<Vec<Option<usize>>>,
    children: Vec<usize>
}

impl BinaryTrie {
    fn new() -> Self {
        BinaryTrie { nodes: vec![vec![None, None]], children: vec![0] }
    }

    /// Inserts given value to Trie and returns number of value after insertion.
    fn insert(&mut self, value: i64) -> usize {
        let mut now = 0;
        for b in (0..32).rev() {
            self.children[now] += 1;
            let eidx = ((value>>b)&1) as usize;
            if let Some(to_idx) = self.nodes[now][eidx] {
                now = to_idx;
            } else {
                self.nodes[now][eidx] = Some(self.nodes.len());
                now = self.nodes.len();
                self.nodes.push(vec![None, None]);
                self.children.push(0);
            }
        }
        self.children[now] += 1;

        self.children[now]
    }

    /// If flag = 0, finds minimum value with xor.
    /// If flag = 1, finds maximum value with xor.
    fn minmax_xor(&mut self, value: i64, flag: i64) -> i64 {
        assert!(self.children[0] >= 1);

        let mut now = 0;
        let mut max_value = 0i64;
        for b in (0..32).rev() {
            let eidx = (((value>>b)&1) ^ flag) as usize;
            if let Some(to_idx) = self.nodes[now][eidx] {
                max_value ^= flag<<b;
                now = to_idx;
            } else if let Some(to_idx) = self.nodes[now][eidx^1] {
                max_value ^= (1^flag)<<b;
                now = to_idx;
            } else {
                unreachable!("Child node is not found: possibly broken data?");
            }
        }
        max_value
    }

    fn remove(&mut self, value: i64) -> bool {
        let mut node_history = vec![];

        let mut now = 0;
        for b in (0..32).rev() {
            node_history.push(now);

            let eidx = ((value>>b)&1) as usize;
            if let Some(to_idx) = self.nodes[now][eidx] {
                now = to_idx;
            } else {
                return false;
            }
        }
        node_history.push(now);

        node_history.reverse();

        let ln = node_history.len();
        for i in 0..ln {
            let n = node_history[i];
            self.children[n] -= 1;
            if i >= 1 {
                let c = node_history[i-1];
                 if self.children[c] == 0 {
                    for w in 0..2 {
                        if let Some(v) = self.nodes[n][w] {
                            if v == c {
                                self.nodes[n][w].take();
                            }
                        }
                    }
                }
            }
        }
        true
    }
}

fn gen(a: &Vec<i64>) -> Vec<i64> {
    let n = a.len();
    let mut w = vec![0; n];
    for i in 0..n {
        w[i] = a[i];
        if i >= 1 {
            w[i] ^= w[i-1];
        }
    }
    w.sort();
    w
}

fn test(ans: &Vec<i64>, original: &Vec<i64>) {
    assert_eq!(gen(ans), gen(original));
}

fn main() {
    input! {
        n: usize, a: [i64; n]
    };
    if n <= 2 {
        println!("{}", a.into_iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" "));
        return;
    }

    // let mut a = vec![0i64; n];
    // for i in 0..n {
    //     a[i] = (1000000000000000000i64 + ((i * i) as i64)) % ((1e9 as i64) + 7);
    // }

    let mut trie = BinaryTrie::new();

    let mut wsum = vec![0; n];
    for i in 0..n {
        wsum[i] = a[i];
        if i >= 1 {
            wsum[i] ^= wsum[i-1];
        }
    }

    let mut ans = vec![];
    {
        let mut best = (1e18 as i64, n);
        for i in 0..n-1 {
            if wsum[i] < best.0 {
                best = (wsum[i], i);
            }
        }
        ans.push(best.0);
        for i in 0..n-1 {
            if i != best.1 {
                trie.insert(wsum[i]);
            }
        }
    }

    let mut last_sum = ans[0];
    for i in 1..n-1 {
        let best = trie.minmax_xor(last_sum, 0);
        ans.push(best);
        assert!(trie.remove(last_sum ^ best));
        last_sum ^= ans[i];
    }
    ans.push(wsum[n-1] ^ last_sum);

    for i in 0..n {
        if i >= 1 {
            print!(" ")
        }
        print!("{}", ans[i]);
    }
    println!();

    test(&ans, &a);
}
