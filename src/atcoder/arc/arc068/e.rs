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

trait Monoid {
    fn mul(&self, _: Self) -> Self;

    fn one() -> Self;
}

struct SegmentTree<T> {
    n: usize,
    data: Vec<T>
}

impl<T: Monoid + Clone + Copy> SegmentTree<T> {
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

    fn get_point(&mut self, idx: usize) -> T {
        let mut pos = self.data.len() / 2 - 1 + idx;
        let mut value = self.data[pos];
        while pos >= 1 {
            let to = (pos - 1) / 2;
            value = value.mul(self.data[to]);
            pos = to;
        }
        value
    }

    fn add_range(&mut self, l: usize, r: usize, value: T) {
        let n = self.data.len() / 2;
        self.range2(l, r, value,0, 0, n);
    }

    fn range2(&mut self, l: usize, r: usize, value: T, idx: usize, segl: usize, segr: usize) {
        if r <= segl || segr <= l {
            return;
        }
        if l <= segl && segr <= r {
            self.data[idx] = self.data[idx].mul(value);
            return;
        }
        let med = (segl + segr) / 2;
        self.range2(l, r, value, idx*2+1, segl, med);
        self.range2(l, r, value, idx*2+2, med, segr);
    }
}


// ===

#[derive(Clone, Debug, Copy)]
struct I32(i32);

impl Monoid for I32 {
    fn mul(&self, other: Self) -> Self {
        return I32(self.0 + other.0)
    }

    fn one() -> Self { I32(0) }
}

fn main() {
    let (n, m): (usize, usize) = read();

    let mut range: Vec<(usize, usize)> = readn(n);
    range.sort_by_key(|a| a.1 - a.0 + 1);

    let mut seg = SegmentTree::new(m+10, I32(0));

    let mut less_idx = 0;
    for t in 1..m+1 {
        while less_idx < n && range[less_idx].1 - range[less_idx].0 + 1 < t {
            seg.add_range(range[less_idx].0, range[less_idx].1+1, I32(1));
            less_idx += 1;
        }
        let mut ans = n - less_idx;


        let mut pt = 0;
        loop {
            pt += t;
            if pt > m {
                break;
            }
            ans += seg.get_point(pt).0 as usize;
        }
        println!("{}", ans);
    }
}