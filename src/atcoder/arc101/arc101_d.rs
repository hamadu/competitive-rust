// https://atcoder.jp/contests/arc101/tasks/arc101_d
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

const MOD: i64 = 1e9 as i64 + 7;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct ModInt {
    value: i64
}

impl ModInt {
    fn new<T: Into<i64>>(value: T) -> Self {
        ModInt { value: value.into() % MOD }
    }
}

impl std::default::Default for ModInt {
    fn default() -> Self {
        ModInt { value: 0 }
    }
}

impl std::ops::Add for ModInt {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut sum = self.value + other.value;
        if sum >= MOD {
            sum -= MOD;
        }
        ModInt { value: sum }
    }
}

impl std::ops::Sub for ModInt {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut sum = self.value - other.value;
        if sum < 0 {
            sum += MOD;
        }
        ModInt { value: sum }
    }
}

impl std::ops::Mul for ModInt {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        ModInt { value: self.value * other.value % MOD }
    }
}

impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, rhs: ModInt) {
        *self = *self + rhs;
    }
}

impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, rhs: ModInt) {
        *self = *self - rhs;
    }
}

impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, rhs: ModInt) {
        *self = *self * rhs;
    }
}


// ===

const INF: i64 = 1e18 as i64;

fn main() {
    input! {
        n: usize, m: usize,
        robots: [i64; n],
        exits: [i64; m],
    };

    let mut set = Treap::new();
    set.push(-INF);
    set.push(INF);
    for e in exits {
        set.push(e);
    }


    let mut pts = vec![];
    for x in robots {
        let lower = set.peek_less(x).unwrap();
        let higher = set.peek_greater(x).unwrap();
        if lower == -INF || higher == INF {
            continue;
        }
        pts.push((x-lower, higher-x));
    }
    let mut ys = pts.clone().into_iter().map(|w| w.1).collect::<Vec<_>>();
    ys.push(0);
    ys.sort();
    ys.dedup();
    let mut ymap = HashMap::new();
    for i in 0..ys.len() {
        ymap.insert(ys[i], i);
    }

    pts.sort_by_key(|w| (w.0, -w.1));
    pts.dedup();
    let n = pts.len();
    let mut dp = FenwickTree::new(100010, ModInt::new(0));
    dp.add(0, ModInt::new(1));

    let mut fr = 0;
    while fr < n {
        let mut to = fr;
        while to < n && pts[fr].0 == pts[to].0 {
            // debug!(pts[to]);
            let yidx = *ymap.get(&pts[to].1).unwrap();
            let lower = dp.range(0..yidx);
            dp.add(yidx, lower);
            to += 1;
        }
        fr = to;
    }

    println!("{}", dp.range(0..100010).value);
}
