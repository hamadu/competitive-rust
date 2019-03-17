#![allow(unused_imports)]
use std::io::*;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;
use std::time::{Duration, SystemTime, Instant};

pub trait InputValue {
    fn parse(s: &str) -> Self;
}

pub fn read<T: InputValue>() -> T {
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    T::parse(&buf.trim())
}

pub fn readn<T: InputValue>(n: usize) -> Vec<T> {
    let mut vec = vec![];
    for _ in 0..n {
        vec.push(read());
    }
    vec
}

pub fn readnc<T: InputValue>() -> Vec<T> {
    let mut vec = vec![];
    let line: String = read();
    for token in line.split_whitespace() {
        vec.push(T::parse(token));
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
parse_tuple!(A, B, C, D);
parse_tuple!(A, B, C, D, E);

// ===

struct XorShift {
    x: usize,
    y: usize,
    z: usize,
    w: usize
}

impl XorShift {
    fn new() -> XorShift {
        XorShift {
            x: 123456789,
            y: 362436069,
            z: 521288629,
            w: 88675123
        }
    }

    fn rotate(&mut self) {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = (self.w ^ (self.w >> 19)) ^ (t ^ (t >> 8));
    }

    fn next_int(&mut self, n: usize) -> usize {
        self.rotate();
        let r = (self.w as i32) % (n as i32);
        if r < 0 {
            (r + n as i32) as usize
        } else {
            r as usize
        }
    }
}


// ===

const SIZE: usize = 30;
const TIMEOUT_MILLIS: u64 = 9800;

struct GlobalState {
    start_time: Instant,
    random: XorShift,
    map: Vec<Vec<i32>>,
    best_ops: Vec<(usize,usize)>
}

impl GlobalState {
    fn exec(&mut self) {
        let timeout = Duration::from_millis(TIMEOUT_MILLIS);

        let mut best_operations: Vec<(usize, usize)> = vec![];
        let mut tried = 0;
        loop {
            if self.start_time.elapsed() > timeout {
                break;
            }
            tried += 1;
            let (ith, newop, score) = self.doit();
            if best_score > score {
                best_score = score;
                apply(&mut best_map, &best_operations[ith], -1);
                apply(&mut best_map, &newop, 1);
                best_operations[ith] = newop;
            }
        }

        println_stderr!("tries: {}", tried);
        println_stderr!("score: {}", best_score);

        for op in best_operations {
            println!("{} {}", op.0, op.1);
        }
    }

    fn doit(&mut self) {
        

         // 900*50

    }
}

fn main() {
    let start_time = Instant::now();
    let mut random = XorShift::new();
    let mut table: Vec<Vec<i32>> = vec![];
    for i in 0..SIZE {
        table.push(readnc());
    }
    GlobalState { start_time: start_time, random: random, map: table, best_ops: vec![] }.exec();
}
