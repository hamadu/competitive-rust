#![allow(unused_imports, unused_variables, dead_code)]
use std::io::*;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;

trait InputValue {
    fn parse(s: &str) -> Self;
}

fn read<T: InputValue>() -> T {
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    T::parse(&buf.trim())
}

fn readnc<T: InputValue>() -> Vec<T> {
    let mut vec = vec![];
    let line: String = read();
    for token in line.split_whitespace() {
        vec.push(T::parse(token));
    }
    vec
}

fn readn<T: InputValue>(n: usize) -> Vec<T> {
    let mut vec = vec![];
    for _ in 0..n {
        vec.push(read());
    }
    vec
}

macro_rules! parse_single_value {
    ($($t:ty),*) => {
        $(
            impl InputValue for $t {
                fn parse(s: &str) -> $t { s.parse().unwrap() }
            }
        )*
	}
}
parse_single_value!(i32, i64, f32, f64, usize, String);

macro_rules! parse_tuple {
	($($t:ident),*) => {
		impl<$($t),*> InputValue for ($($t),*) where $($t: InputValue),* {
			fn parse(s: &str) -> ($($t),*) {
				let mut tokens = s.split_whitespace();
				let t = ($($t::parse(tokens.next().unwrap())),*);
				t
			}
		}
	}
}
parse_tuple!(A, B);
parse_tuple!(A, B, C);

// ===

trait SemiRing {
    fn add(&self, Self) -> Self;

    fn zero() -> Self;

    fn mul(&self, Self) -> Self;

    fn one() -> Self;
}

struct SegmentTree<T> {
    n: usize,
    data: Vec<T>,
    add: Vec<T>
}

impl<T: SemiRing + Clone + Copy + PartialEq> SegmentTree<T> {
    fn new(n: usize, initial: T) -> Self {
        let vs = (n-1).next_power_of_two() << 1;
        SegmentTree { n: n, data: vec![initial; vs], add: vec![T::zero(); vs] }
    }

    fn new_with(v: &Vec<T>) -> Self {
        let vs = max(4, (v.len()-1).next_power_of_two() << 1);
        let n = v.len();
        let mut data: Vec<T> = vec![T::one(); vs];

        let bottom = vs/2-1;
        for i in 0..n {
            data[bottom+i] = v[i].clone();
        }
        for i in (0..bottom).rev() {
            data[i] = data[i*2+1].mul(data[i*2+2].clone());
        }
        SegmentTree { n: v.len(), data: data, add: vec![T::zero(); vs] }
    }

    fn change(&mut self, idx: usize, new_value: T) {
        let mut pos = self.data.len() / 2 - 1 + idx;
        self.data[pos] = new_value;
        while pos >= 1 {
            let to = (pos - 1) / 2;
            self.data[to] = self.data[to*2+1].mul(self.data[to*2+2].clone());
            pos = to;
        }
    }

    fn add_range(&mut self, l: usize, r: usize, v: T) {
        let l2 = self.data.len() / 2;
        self.range3(l, r, v, 0, 0, l2)
    }

    fn range3(&mut self, l: usize, r: usize, v: T, idx: usize, segl: usize, segr: usize) {
        if r <= segl || segr <= l {
            return
        }
        if l <= segl && segr <= r {
            self.add[idx] = self.add[idx].add(v);
            return
        }

        let med = (segl + segr) / 2;
        self.range3(l, r, v, idx*2+1, segl, med);
        self.range3(l, r, v, idx*2+2, med, segr);
        let l = self.data[idx*2+1].add(self.add[idx*2+1]);
        let r = self.data[idx*2+2].add(self.add[idx*2+2]);
        self.data[idx] = l.mul(r);
    }

    fn mul_range(&mut self, l: usize, r: usize) -> T {
        let l2 = self.data.len() / 2;
        self.range2(l, r, 0, 0, l2)
    }

    fn range2(&mut self, l: usize, r: usize, idx: usize, segl: usize, segr: usize) -> T {
        if r <= segl || segr <= l {
            return T::one()
        }
        if l <= segl && segr <= r {
            return self.data[idx].add(self.add[idx])
        }
        let med = (segl + segr) / 2;
        let ret = self.range2(l, r, idx*2+1, segl, med).mul(self.range2(l, r, idx*2+2, med, segr));
        ret.add(self.add[idx])
    }
}

// ===

#[derive(Clone, Debug, Copy, PartialEq)]
struct I32(i32);

impl SemiRing for I32 {
    fn add(&self, other: Self) -> Self {
        I32(self.0 + other.0)
    }

    fn zero() -> Self { I32(0) }

    fn mul(&self, other: Self) -> Self {
        if self.0 > other.0 { I32(self.0) } else { I32(other.0) }
    }

    fn one() -> Self { I32(-INF) }
}

// ===

const INF: i32 = 1000000000;

fn main() {
    let (n, m): (usize, usize) = read();
    let mut tak: Vec<(usize, usize)> = readn(n);
    tak.sort();

    let mut ans = if n < m { 0 } else { n - m } as i32;
    let mut seg = SegmentTree::new(m+10, I32::one());
    for l in 1..m+2 {
        seg.change(l, I32(-((m+1-l) as i32)));
    }

    let mut head = 0;
    for l in 0..m+1 {
        while head < n && tak[head].0 == l {
            // println!("{} {}", 0, tak[head].1+1);
            seg.add_range(0, tak[head].1+1, I32(1));
            head += 1;
        }
        let best = seg.mul_range(l+2, m+2).0 - l as i32;
        ans = max(ans, best)
    }
    println!("{}", ans);
}