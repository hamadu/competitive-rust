// https://atcoder.jp/contests/nikkei2019-final/tasks/nikkei2019_final_f
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

// =====

use std::rc::Rc;
type Merger<T> = Rc<Fn(T, T) -> T>;

struct SegmentTree<T> {
    n: usize,
    default: T,
    bottom_offset: usize,
    data: Vec<T>,
    lazy: Vec<T>,
    merger: Merger<T>,
    lazy_merger: Merger<T>,
}

impl<T: Copy + Eq> SegmentTree<T> {
    fn new(n: usize, initial: T, merger: Merger<T>, lazy_merger: Merger<T>) -> Self {
        let z = (8u32 * std::mem::size_of::<usize>() as u32) - n.leading_zeros();
        let size = (1<<z) as usize;
        SegmentTree {
            n: size*2,
            default: initial,
            bottom_offset: size,
            data: vec![initial; size*2],
            lazy: vec![initial; size*2],
            merger: merger,
            lazy_merger: lazy_merger
        }
    }

    /// Compute value [i, j).
    fn compute_range(&mut self, i: usize, j: usize) -> T {
        let m = self.bottom_offset;
        self.inner_compute_range(i, j, 1, 0, m)
    }

    fn inner_compute_range(&mut self, i: usize, j: usize, idx: usize, begin: usize, end: usize) -> T {
        self.propagate(idx);

        if end <= i || j <= begin {
            return self.default;
        } else if i <= begin && end <= j {
            return self.data[idx];
        }
        let center = (begin+end)/2;
        let l = self.inner_compute_range(i, j, idx*2, begin, center);
        let r = self.inner_compute_range(i, j, idx*2+1, center, end);
        (self.merger)(l, r)
    }

    /// Apply given value to [i, j).
    fn apply_range(&mut self, i: usize, j: usize, value: T) {
        let m = self.bottom_offset;
        self.inner_apply_range(i, j, value, 1, 0, m);
    }

    fn inner_apply_range(&mut self, i: usize, j: usize, value: T, idx: usize, begin: usize, end: usize) {
        self.propagate(idx);

        if end <= i || j <= begin {
            return;
        } else if i <= begin && end <= j {
            self.lazy[idx] = (self.lazy_merger)(self.lazy[idx], value);
            self.propagate(idx);
            return;
        }
        let center = (begin+end)/2;
        self.inner_apply_range(i, j, value, idx*2, begin, center);
        self.inner_apply_range(i, j, value, idx*2+1, center, end);
        self.data[idx] = (self.merger)(self.data[idx*2], self.data[idx*2+1]);
    }

    fn merge(&mut self, i: usize, j: usize) -> T {
        (self.merger)(self.data[i], self.data[j])
    }

    fn propagate(&mut self, idx: usize) {
        if self.lazy[idx] == self.default {
            return;
        }
        self.data[idx] = (self.lazy_merger)(self.data[idx], self.lazy[idx]);
        if idx*2+1 < self.lazy.len() {
            self.lazy[idx*2] = (self.lazy_merger)(self.lazy[idx*2], self.lazy[idx]);
            self.lazy[idx*2+1] = (self.lazy_merger)(self.lazy[idx*2+1], self.lazy[idx]);
        }
        self.lazy[idx] = self.default;
    }
}



// =====

type City = (i64, i64, i64, usize);

const INF: i64 = 1e18 as i64;

fn solve(s: usize, t: usize, mut cities: Vec<City>) -> i64 {
    if cities[s].1 < cities[t].1 {
        return solve(t, s, cities);
    }
    let n = cities.len();
    let mut sx = cities[s].0;
    let mut sy = cities[s].1;
    let mut scost = cities[s].2;
    let mut tx = cities[t].0;
    let mut ty = cities[t].1;
    let mut tcost = cities[t].2;

    if cities[t].0 <= cities[s].0 {
        let mut best = cities[s].2;
        for (x,y,cost,_) in cities {
            if sx <= x && sy <= y {
                best = min(best, cost*2);
            }
        }
        return best;
    }

    cities.sort();
    // cities.sort_by_key(|&(x,y,_,_)| (x,-y));

    let mut ys = cities.iter().map(|&c| c.1).collect::<Vec<_>>();
    ys.sort();
    ys.dedup();
    let mut ymap = HashMap::new();
    for i in 0..ys.len() {
        ymap.insert(ys[i], i);
    }

    let mut segtree = SegmentTree::new(100010, INF, Rc::new(|i, j| min(i, j)), Rc::new(|i, j| min(i, j)));

    let mut iter = 0;
    let mut lower_y = 0;
    while iter < n && cities[iter].0 <= sx {
        if cities[iter].1 < sy {
            let yid = *ymap.get(&cities[iter].1).unwrap();
            segtree.apply_range(yid, yid+1, scost);
        } else if cities[iter].1 == sy {
            let yid = *ymap.get(&cities[iter].1).unwrap();
            lower_y = yid;
            segtree.apply_range(yid, yid+1, 0);
        }
        iter += 1;
    }

    let mut answer = INF;
    while iter < n {
        let id = cities[iter].3;
        let cost = cities[iter].2;

        let yid = *ymap.get(&cities[iter].1).unwrap();
        lower_y = min(lower_y, yid);
        let best = segtree.compute_range(0, yid+1) + cost;
        segtree.apply_range(yid, yid+1, best);
        segtree.apply_range(lower_y, yid+1, best + cost);

        if id == t {
            // debug!(best);
            answer = min(answer, best);
        } else if tx <= cities[iter].0 && ty <= cities[iter].1 {
            // debug!(best,cost);
            answer = min(answer, best + cost);
        }
        iter += 1;
    }

    answer
}

fn main() {
    // {
    //     let mut segtree = SegmentTree::new(100010, INF, Rc::new(|i, j| min(i, j)), Rc::new(|i, j| min(i, j)));
    //     segtree.apply_range(1, 3, 10);
    //     debug!(segtree.compute_range(0, 1));
    //     debug!(segtree.compute_range(1, 2));
    //     debug!(segtree.compute_range(2, 3));
    //     debug!("==");
    //     segtree.apply_range(0, 6, 15);
    //     debug!(segtree.compute_range(0, 1));
    //     debug!(segtree.compute_range(1, 2));
    //     debug!(segtree.compute_range(2, 3));
    //     debug!(segtree.compute_range(3, 4));
    //     debug!(segtree.compute_range(4, 5));
    // }

    input! {
        n: usize, s: usize1, t: usize1,
        cities: [(i64, i64, i64); n]
    };
    let cities = cities.into_iter().zip(0..n).map(|(a,b)| (a.0, a.1, a.2, b)).collect::<Vec<_>>();
    println!("{}", solve(s, t, cities));
}
