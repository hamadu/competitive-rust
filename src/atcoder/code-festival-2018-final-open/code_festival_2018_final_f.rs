// https://atcoder.jp/contests/code-festival-2018-final-open/tasks/code_festival_2018_final_f
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


use std::rc::Rc;
type Merger<T> = Rc<Fn(T, T) -> T>;

struct SegmentTree<T> {
    n: usize,
    default: T,
    bottom_offset: usize,
    data: Vec<T>,
    merger: Merger<T>
}

impl<T: Copy> SegmentTree<T> {
    fn new(n: usize, initial: T, merger: Merger<T>) -> Self {
        let z = (8u32 * std::mem::size_of::<usize>() as u32) - n.leading_zeros();
        let size = (1<<z) as usize;
        SegmentTree { n: size*2, default: initial, bottom_offset: size, data: vec![initial; size*2], merger: merger }
    }

    fn get(&self, i: usize) -> T {
        let mut ret = self.default;
        let mut idx = i + self.bottom_offset;
        while idx >= 1 {
            ret = (self.merger)(ret, self.data[idx]);
            idx /= 2;
        }
        ret
    }

    fn set(&mut self, i: usize, value: T) {
        self.data[self.bottom_offset+i] = value;
    }

    /// Apply given value to [i, j).
    fn apply_range(&mut self, i: usize, j: usize, value: T) {
        let m = self.bottom_offset;
        self.range(i, j, value, 1, 0, m);
    }

    fn range(&mut self, i: usize, j: usize, value: T, idx: usize, begin: usize, end: usize) {
        if end <= i || j <= begin {
            return;
        } else if i <= begin && end <= j {
            self.data[idx] = (self.merger)(self.data[idx], value);
            return;
        }
        let center = (begin+end)/2;
        self.range(i, j, value, idx*2, begin, center);
        self.range(i, j, value, idx*2+1, center, end);
    }

    fn merge(&mut self, i: usize, j: usize) -> T {
        (self.merger)(self.data[i], self.data[j])
    }
}


/// Fenwick tree.
struct FenwickTree<T> {
    data: Vec<T>
}

impl<T: Copy + Default + std::ops::Add + std::ops::AddAssign + std::ops::Sub> FenwickTree<T> {
    fn new(n: usize, initial: T) -> Self {
        FenwickTree { data: vec![initial; n+1] }
    }

    /// Computes sum value of ([0, i)).
    fn sum(&self, i: usize) -> T {
        let mut idx = i as i64;
        let mut ret = T::default();
        while idx > 0 {
            ret += self.data[idx as usize];
            idx -= idx & (-idx);
        }
        ret
    }

    /// Adds value x into i-th position.
    fn add(&mut self, i: usize, x: T) {
        let mut idx = (i + 1) as i64;
        while idx < self.data.len() as i64 {
            self.data[idx as usize] += x;
            idx += idx & (-idx);
        }
    }

    fn range(&self, r: std::ops::Range<usize>) -> <T as std::ops::Sub>::Output {
        self.sum(r.end) - self.sum(r.start)
    }
}



// ====

#[derive(Debug, Clone)]
struct Treap<T> {
    total: usize,
    root: Link<T>
}

impl<T: Ord + Copy + Debug> Treap<T> {
    fn new() -> Self {
        Treap {
            total: 0,
            root: None
        }
    }

    fn push(&mut self, v: T) -> bool {
        if self.contains(v) {
            return false;
        }
        let u = self.root.take();
        self.root = Node::push(u, v);
        self.total += 1;
        true
    }

    fn remove(&mut self, v: T) -> bool {
        let r = Node::remove(&mut self.root, v);
        if r { self.total -= 1; }
        r
    }

    fn len(&self) -> usize {
        self.total
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

    fn merge(left: Link<T>, right: Link<T>) -> Link<T> {
        match (left, right) {
            (None, None) => None,
            (None, x) => x,
            (x, None) => x,
            (Some(l), Some(r)) => {
                if l.priority < r.priority {
                    let mut l = l;
                    l.right = Self::merge(l.right.take(), Some(r));
                    Some(l)
                } else {
                    let mut r = r;
                    r.left = Self::merge(Some(l), r.left.take());
                    Some(r)
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
                    (nl, Some(x))
                } else {
                    let (nl, nr) = Self::split_at(x.right.take(), k-1-lsz);
                    x.right = nl;
                    (Some(x), nr)
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
                    (nl, Some(x))
                } else {
                    let (nl, nr) = Self::split_by(x.right.take(), v);
                    x.right = nl;
                    (Some(x), nr)
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
        if both_none {
            now.take();
        }
        w
    }
}

// ====

const INF: i64 = 1e18 as i64;

fn main() {
    input! {
        n: usize, k: usize,
        meals: [(i32, i64); n]
    };


    // let mut seg = SegmentTree::new(k+2*n, 0i64, Rc::new(|i, j| i + j));
    // let mut fen = FenwickTree::new(k+2*n, 0i64);

    let mut ram = Treap::new();
    let mut res = Treap::new();
    for i in 0..k {
        let pv = i as i32;
        ram.push((INF, -pv-1));
    }

    let mut total = 0;
    for ((kind, value), idx) in meals.into_iter().zip(0..n) {
        let idx = idx as i32;
        total += value;
        if kind == 0 {
            ram.push((value, idx));
            if res.len() >= 1 {
                let best = res.max().unwrap();
                res.remove(best);
            } else {
                let ram_cancel = ram.min().unwrap();
                ram.remove(ram_cancel);
                total -= ram_cancel.0;
            }
        } else {
            res.push((value, idx));
            if ram.len() >= 1 {
                let best = ram.max().unwrap();
                ram.remove(best);
            } else {
                let res_cancel = res.min().unwrap();
                res.remove(res_cancel);
                total -= res_cancel.0;
            }
        }
    }
    println!("{}", total);
}
