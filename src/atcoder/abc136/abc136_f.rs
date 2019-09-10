// https://atcoder.jp/contests/abc136/tasks/abc136_f
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


// ====

#[derive(Debug, Clone)]
struct Treap<T> {
    root: Link<T>
}

impl<T: Ord + Copy + Debug> Treap<T> {
    fn new() -> Self {
        Treap {
            root: None
        }
    }

    fn push(&mut self, v: T) -> bool {
        if self.contains(v) {
            return false;
        }
        let u = self.root.take();
        self.root = Node::push(u, v);
        true
    }

    fn remove(&mut self, v: T) -> bool {
        Node::remove(&mut self.root, v)
    }

    fn len(&self) -> usize {
        Node::sz(&self.root)
    }

    fn min(&self) -> Option<T> {
        Node::min(&self.root)
    }

    fn max(&self) -> Option<T> {
        Node::max(&self.root)
    }

    fn contains(&self, value: T) -> bool {
        Node::contains(&self.root, value)
    }

    fn peek_less(&self, value: T) -> Option<T> {
        Node::peek_less(&self.root, value)
    }

    fn count_less(&self, value: T) -> usize {
        Node::count_less(&self.root, value)
    }

    fn peek_greater(&self, value: T) -> Option<T> {
        Node::peek_greater(&self.root, value)
    }
}

struct XorShift {
    x: i64,
    y: i64,
    z: i64,
    w: i64
}

impl XorShift {
    fn rotate(&mut self) {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = (self.w ^ (self.w >> 19)) ^ (t ^ (t >> 8));
    }

    fn next_i64(&mut self) -> i64 {
        self.rotate();
        self.w
    }
}

static mut RAND: XorShift = XorShift {
    x: 123456789i64,
    y: 362436069i64,
    z: 521288629i64,
    w: 88675123i64
};

fn generate_next() -> i64 {
    let x;
    unsafe {
        x = RAND.next_i64();
    }
    x
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    size: usize,
    priority: i64,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Ord + Copy + Debug> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value: value,
            size: 1,
            priority: generate_next(),
            left: None,
            right: None
        }
    }

    fn sz(node: &Link<T>) -> usize {
        match node.as_ref() { // TODO: Remove as_ref() after upgrading above 1.26
            None => 0,
            Some(x) => x.size
        }
    }

    fn calc(node: &mut Link<T>) {
        if let Some(x) = node.as_mut() { // TODO: Remove as_mut() after upgrading above 1.26
            x.size = 1 + Node::sz(&x.left) + Node::sz(&x.right);
        }
    }

    fn merge(left: Link<T>, right: Link<T>) -> Link<T> {
        match (left, right) {
            (None, None) => None,
            (None, x) => x,
            (x, None) => x,
            (Some(l), Some(r)) => {
                if l.priority < r.priority {
                    let mut l = l;
                    l.right = Self::merge(l.right.take(), Some(r));
                    let mut l = Some(l);
                    Node::calc(&mut l);
                    l
                } else {
                    let mut r = r;
                    r.left = Self::merge(Some(l), r.left.take());
                    let mut r = Some(r);
                    Node::calc(&mut r);
                    r
                }
            }
        }
    }

    fn min(now: &Link<T>) -> Option<T> {
        match now.as_ref() { // TODO: Remove as_ref() after upgrading above 1.26
            None    => None,
            Some(x) => {
                match x.left.as_ref() { // TODO: Remove as_ref() after upgrading above 1.26
                    None    => Some(x.value),
                    Some(_) => Self::min(&x.left)
                }
            }
        }
    }

    fn max(now: &Link<T>) -> Option<T> {
        match now.as_ref() { // TODO: Remove as_ref() after upgrading above 1.26
            None    => None,
            Some(x) => {
                match x.right.as_ref() { // TODO: Remove as_ref() after upgrading above 1.26
                    None    => Some(x.value),
                    Some(_) => Self::max(&x.right)
                }
            }
        }
    }

    fn contains(now: &Link<T>, value: T) -> bool {
        match now.as_ref() { // TODO: Remove as_ref() after upgrading above 1.26
            None => false,
            Some(x) => {
                match value.cmp(&x.value) {
                    Ordering::Less    => Self::contains(&x.left, value),
                    Ordering::Equal   => true,
                    Ordering::Greater => Self::contains(&x.right, value)
                }
            }
        }
    }

    fn count_less(now: &Link<T>, value: T) -> usize {
        match now.as_ref() { // TODO: Remove as_ref() after upgrading above 1.26
            None => 0,
            Some(x) => {
                match value.cmp(&x.value) {
                    Ordering::Less | Ordering::Equal => {
                        Self::count_less(&x.left, value)
                    },
                    Ordering::Greater => {
                        let w = Self::count_less(&x.right, value);
                        Node::sz(&x.left) + 1 + w
                    }
                }
            }
        }
    }

    fn peek_less(now: &Link<T>, value: T) -> Option<T> {
        match now.as_ref() { // TODO: Remove as_ref() after upgrading above 1.26
            None => None,
            Some(x) => {
                match value.cmp(&x.value) {
                    Ordering::Less | Ordering::Equal => {
                        Self::peek_less(&x.left, value)
                    },
                    Ordering::Greater => {
                        let w = Self::peek_less(&x.right, value);
                        if w == None {
                            Some(x.value)
                        } else {
                            Some(max(w.unwrap(), x.value))
                        }
                    }
                }
            }
        }
    }

    fn peek_greater(now: &Link<T>, value: T) -> Option<T> {
        match now.as_ref() { // TODO: Remove as_ref() after upgrading above 1.26
            None => None,
            Some(x) => {
                match value.cmp(&x.value) {
                    Ordering::Greater | Ordering::Equal => {
                        Self::peek_greater(&x.right, value)
                    },
                    Ordering::Less => {
                        let w = Self::peek_greater(&x.left, value);
                        if w == None {
                            Some(x.value)
                        } else {
                            Some(min(w.unwrap(), x.value))
                        }
                    }
                }
            }
        }
    }

    fn split_at(now: Link<T>, k: usize) -> (Link<T>, Link<T>) {
        match now {
            None => (None, None),
            Some(x) => {
                let mut x = x;
                let lsz = Self::sz(&x.left);
                if k <= lsz {
                    let (nl, nr) = Self::split_at(x.left.take(), k);
                    x.left = nr;
                    let mut x = Some(x);
                    Node::calc(&mut x);
                    (nl, x)
                } else {
                    let (nl, nr) = Self::split_at(x.right.take(), k-1-lsz);
                    x.right = nl;
                    let mut x = Some(x);
                    Node::calc(&mut x);
                    (x, nr)
                }
            },
        }
    }

    fn split_by(now: Link<T>, v: T) -> (Link<T>, Link<T>) {
        match now {
            None => (None, None),
            Some(x) => {
                let mut x = x;
                if v <= x.value {
                    let (nl, nr) = Self::split_by(x.left.take(), v);
                    x.left = nr;
                    let mut x = Some(x);
                    Node::calc(&mut x);
                    (nl, x)
                } else {
                    let (nl, nr) = Self::split_by(x.right.take(), v);
                    x.right = nl;
                    let mut x = Some(x);
                    Node::calc(&mut x);
                    (x, nr)
                }
            },
        }
    }

    fn push(now: Link<T>, new_value: T) -> Link<T> {
        let new_node = Some(Box::new(Node::new(new_value)));
        let (left, right) = Self::split_by(now, new_value);
        Self::merge(Self::merge(left, new_node), right)
    }

    fn remove(now: &mut Link<T>, target_value: T) -> bool {
        let mut both_none = false;
        let w = match now.as_mut() { // TODO: Remove as_mut() after upgrading above 1.26
            None => false,
            Some(x) => {
                if target_value < x.value {
                    Self::remove(&mut x.left, target_value)
                } else if x.value < target_value {
                    Self::remove(&mut x.right, target_value)
                } else {
                    let new_link = Self::merge(x.left.take(), x.right.take());
                    if let Some(node) = new_link {
                        *x = node;
                    } else {
                        both_none = true;
                    }
                    true
                }
            }
        };
        Node::calc(now);
        if both_none {
            now.take();
        }
        w
    }
}

// ====

type Point = (i64, i64, usize, i64, i64);

const MOD: i64 = 998244353i64;

fn main() {
    input! {
        n: usize,
        points: [(i64, i64); n]
    };

    let mut points = points.into_iter().zip(0..n).map(|((x, y), idx)| {
        let xx = x * 1000000 + idx as i64;
        let yy = y * 1000000 + idx as i64;
        (x, y, idx, xx, yy)
    }).collect::<Vec<_>>();

    let mut lrud = vec![vec![0; 8]; n];

    for i in 0..2 {
        points.sort();
        if i == 1 {
            points.reverse();
        }
        let mut treap = Treap::new();
        let mut fr = 0;
        while fr < n {
            let mut to = fr;
            while to < n && points[fr].0 == points[to].0 {
                let idx = points[fr].2;
                let y = points[fr].1 * 1000000 as i64;
                let y1 = y + (n as i64) + 10;
                lrud[idx][0+i] = fr;
                lrud[idx][4+i*2] = treap.count_less(y);
                lrud[idx][5+i*2] = treap.len() - treap.count_less(y1);
                to += 1;
            }
            let mut to = fr;
            while to < n && points[fr].0 == points[to].0 {
                let idx = points[fr].2 as i64;
                let y = points[fr].1 as i64;
                treap.push(points[fr].4);
                to += 1;
            }
            fr = to;
        }
    }

    {
        points.sort_by_key(|p| p.1);
        let mut fr = 0;
        while fr < n {
            let mut to = fr;
            while to < n && points[fr].0 == points[to].0 {
                let idx = points[fr].2;
                lrud[idx][3] = fr;
                to += 1;
            }
            for w in fr..to {
                let idx = points[fr].2;
                lrud[idx][2] = n-to;
            }
            fr = to;
        }
    }


    let mut p2 = vec![1; n+1];
    for i in 1..n+1 {
        p2[i] = p2[i-1] * 2 % MOD;
    }

    let mut ans = 0;
    for i in 0..n {
        let mut ng1 = 1;
        for j in 0..4 {
            let w = lrud[i][j];
            ng1 += p2[w]-1;
            ng1 %= MOD;
        }
        let mut ng2 = 0;
        for j in 4..8 {
            let w = lrud[i][j];
            ng2 += MOD-p2[w]+1;
            ng2 %= MOD;
        }
        // debug!(i, ng1, (MOD-ng2), lrud[i]);
        ans += p2[n] + MOD + MOD - ng1 - ng2;
        ans %= MOD;
    }
    println!("{}", ans);
}
