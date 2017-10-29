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

struct SlopeSet {
    heap: BinaryHeap<i64>,
    add: i64
}

impl SlopeSet {
    fn new() -> Self {
        SlopeSet { heap: BinaryHeap::new(), add: 0 }
    }

    fn add(&mut self, x: i64) {
        self.add += x;
    }

    fn push_left(&mut self, x: i64) {
        let tx = x - self.add;
        self.heap.push(tx);
    }

    fn push_right(&mut self, x: i64) {
        let tx = x - self.add;
        self.heap.push(-tx);
    }

    fn pop_left(&mut self) -> i64 {
        let x = self.heap.pop().unwrap();
        x + self.add
    }

    fn pop_right(&mut self) -> i64 {
        let x = self.heap.pop().unwrap();
        -x + self.add
    }

    fn peek_left_pair(&mut self) -> (i64, i64) {
        let one = self.heap.pop().unwrap();
        let two = self.heap.pop().unwrap();
        self.heap.push(one);
        self.heap.push(two);
        (two + self.add, one + self.add)
    }

    fn peek_right_pair(&mut self) -> (i64, i64) {
        let one = self.heap.pop().unwrap();
        let two = self.heap.pop().unwrap();
        self.heap.push(one);
        self.heap.push(two);
        (-one + self.add, -two + self.add)
    }
}

const INF: i64 = 1e16 as i64;

fn main() {
    let n: usize = read();
    let rects: Vec<(i64, i64)> = readn(n);


    let mut from = rects[0].0;
    let mut to = rects[0].0;
    let mut bottom = 0;

    let mut left = SlopeSet::new();
    left.push_left(-INF);
    left.push_left(from);

    let mut right = SlopeSet::new();
    right.push_right(INF);
    right.push_right(from);


    for i in 1..n {
        let fl = rects[i-1].0;
        let fr = rects[i-1].1;
        let tl = rects[i].0;
        let tr = rects[i].1;
        from -= tr-tl;
        left.add(-tr+tl);
        to += fr-fl;
        right.add(fr-fl);

        let (l0, l1) = left.peek_left_pair();
        let (r0, r1) = right.peek_right_pair();

        let position = tl;
        if position <= l1 {
            from = max(l0, position);
            to = l1;
            bottom += (l1 - position).abs();
            left.push_left(position);
            left.push_left(position);
            let l0 = left.pop_left();
            right.push_right(l0);
        } else if position < r0 {
            from = position;
            to = position;
            left.push_left(position);
            right.push_right(position);
        } else {
            from = r0;
            to = min(r1, position);
            bottom += (position - r0).abs();
            right.push_right(position);
            right.push_right(position);
            let r0 = right.pop_right();
            left.push_left(r0);
        }
    }

    println!("{}", bottom);
}