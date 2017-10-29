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
parse_tuple!(A, B, C, D);

// ===

trait Monoid {
    fn mul(&self, Self) -> Self;

    fn one() -> Self;
}

struct SegmentTree<T> {
    n: usize,
    data: Vec<T>
}

impl<T: Monoid + Clone> SegmentTree<T> {
    fn new(n: usize, initial: T) -> Self {
        let vs = (n-1).next_power_of_two() << 1;
        SegmentTree { n: n, data: vec![initial; vs] }
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
        SegmentTree { n: v.len(), data: data }
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

    fn range(&self, l: usize, r: usize) -> T {
        self.range2(l, r, 0, 0, self.data.len() / 2)
    }

    fn range2(&self, l: usize, r: usize, idx: usize, segl: usize, segr: usize) -> T {
        if r <= segl || segr <= l {
            return T::one()
        }
        if l <= segl && segr <= r {
            return self.data[idx].clone()
        }
        let med = (segl + segr) / 2;
        self.range2(l, r, idx*2+1, segl, med).mul(self.range2(l, r, idx*2+2, med, segr))
    }
}


// ===

#[derive(Clone, Debug, Copy)]
struct I64(i64);

impl Monoid for I64 {
    fn mul(&self, other: Self) -> Self {
        if self.0 < other.0 { I64(self.0) } else { I64(other.0) }
    }

    fn one() -> Self { I64(1e15 as i64) }
}

// ===

fn abs(a: usize, b: usize) -> i64 {
    (if a < b {
        b - a
    } else {
        a - b
    }) as i64
}

fn main() {
    let (n, q, mut a, mut b): (usize, usize, usize, usize) = read();
    a -= 1;
    b -= 1;
    let mut x: Vec<usize> = readnc();
    for i in 0..q {
        x[i] -= 1;
    }
    let mut last = a;

    let mut left = SegmentTree::new(n+10, I64::one());
    let mut right = SegmentTree::new(n+10, I64::one());
    for i in 0..n {
        let lcost = i as i64;
        let rcost = (n - 1 - i) as i64;
        left.change(i, I64(abs(b, i) + lcost));
        right.change(i, I64(abs(b, i) + rcost));
    }

    let mut add_all = 0;
    let mut dp: Vec<i64> = vec![0; n];
    for i in 0..n {
        dp[i] = abs(b, i);
    }

    let mut add_all = 0;
    for i in 0..q {
        let add = abs(last, x[i]);
        let tlast = dp[last] + add_all + add;
        let mut tt = I64::one().0;

        let lcost = x[i] as i64;
        let rcost = (n - 1 - x[i]) as i64;
        tt = min(tt, left.range(x[i], n).0 - lcost + add_all);
        tt = min(tt, right.range(0, x[i]+1).0 - rcost + add_all);
        add_all += add;
        dp[last] = min(tt, tlast) - add_all;


        let lcost = last as i64;
        let rcost = (n - 1 - last) as i64;
        left.change(last, I64(dp[last] + lcost));
        right.change(last, I64(dp[last] + rcost));

        // println!("{:?} {}", dp, add_all);

        last = x[i];
    }

    let mut best = I64::one().0;
    for i in 0..n {
        best = min(best, dp[i] + add_all);
    }
    println!("{}", best);
}