#![allow(unused_imports, unused_variables, dead_code)]
use std::io::*;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;

pub trait InputValue {
    fn parse(s: &str) -> Self;
}

pub fn read<T: InputValue>() -> T {
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    T::parse(&buf.trim())
}

pub fn readnc<T: InputValue>() -> Vec<T> {
    let mut vec = vec![];
    let line: String = read();
    for token in line.split_whitespace() {
        vec.push(T::parse(token));
    }
    vec
}

pub fn readn<T: InputValue>(n: usize) -> Vec<T> {
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
    fn mul(&self, Self) -> Self;

    fn one() -> Self;
}

struct SegmentTree<T> {
    n: usize,
    data: Vec<T>
}

impl<T: Monoid + Clone> SegmentTree<T> {
    fn new(n: usize, initial: T) -> SegmentTree<T> {
        let vs = (n-1).next_power_of_two() << 1;
        SegmentTree { n: n, data: vec![initial; vs] }
    }

    fn new_with(v: &Vec<T>) -> SegmentTree<T> {
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

const INF: i32 = 1000000000;

#[derive(Clone, Debug)]
struct Item {
    value: i32,
    index: usize
}

impl Monoid for Item {
    fn mul(&self, other: Self) -> Self {
        return if self.value < other.value { (*self).clone() } else { other }
    }

    fn one() -> Self {
        Item { value: INF, index: 0 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Range {
    from: usize,
    to: usize,
    value: i32,
    l: usize,
    r: usize,
}

impl std::cmp::Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&(other.value)).reverse()
    }
}

impl std::cmp::PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let n: usize = read();
    let a: Vec<i32> = readnc();

    let items: Vec<Item> = a.into_iter().zip((0..n).into_iter()).map(|(v, idx)| Item { value: v, index: idx }).collect();
    let mut odd_tree: SegmentTree<Item> = SegmentTree::new_with(&items);
    let mut even_tree: SegmentTree<Item> = SegmentTree::new_with(&items);
    for i in 0..n {
        if i % 2 == 0 {
            odd_tree.change(i, Item::one());
        } else {
            even_tree.change(i, Item::one());
        }
    }


    let mut heap: BinaryHeap<Range> = BinaryHeap::new();

    heap.push(compute(0, n, &odd_tree, &even_tree));
    let mut first = true;
    while let Some(range) = heap.pop() {
        //println!("{}/{}", range.l, range.r);
        print!("{}{} {}", if first { "" } else { " " }, items[range.l].value, items[range.r].value);
        for new_range in process(&range, &odd_tree, &even_tree) {
            heap.push(new_range);
        }
        first = false;
    }
    println!();
}

fn process(range: &Range, odd_tree: &SegmentTree<Item>, even_tree: &SegmentTree<Item>) -> Vec<Range> {
    let mut res = vec![];
    if range.from < range.l {
        res.push(compute(range.from, range.l, odd_tree, even_tree));
    }
    if range.r - range.l >= 3 {
        res.push(compute(range.l+1, range.r, odd_tree, even_tree));
    }
    if range.r+1 < range.to {
        res.push(compute(range.r+1, range.to, odd_tree, even_tree));
    }
    res
}

fn compute(l: usize, r: usize, odd_tree: &SegmentTree<Item>, even_tree: &SegmentTree<Item>) -> Range {
    let tree0 = if l % 2 == 0 { even_tree } else { odd_tree };
    let tree1 = if l % 2 == 0 { odd_tree } else { even_tree };
    let item0 = tree0.range(l, r);
    let item1 = tree1.range(item0.index+1, r);
    Range { from: l, to: r, value: item0.value, l: item0.index, r: item1.index }
}