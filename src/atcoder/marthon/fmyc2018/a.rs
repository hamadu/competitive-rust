#![allow(unused_imports, unused_variables, dead_code)]
use std::io::*;
use std::io::{Write, Stderr};
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;
use std::time::{Duration, SystemTime, Instant};

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

macro_rules! println_stderr {
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
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

type TheMap = Vec<Vec<i32>>;

const N: usize = 100;
const Q: usize = 1000;
const TIMEOUT_MILLIS: u64 = 58000;

struct Operation {
    x: usize,
    y: usize,
    h: usize
}

struct GlobalState {
    start_time: Instant,
    random: XorShift,
    map: TheMap
}

impl GlobalState {
    fn exec(&mut self) {
        let timeout = Duration::from_millis(TIMEOUT_MILLIS);
        let mut tried = 0;
        let mut best_operations: Vec<Operation> = vec![];
        let mut best_map = vec![vec![0; N]; N];
        for _ in 0..Q {
            let op = self.generate();
            apply(&mut best_map, &op, 1);
            best_operations.push(op)
        }
        let mut best_score = diff(&self.map, &best_map);

        loop {
            if self.start_time.elapsed() > timeout {
                break;
            }
            tried += 1;
            let (ith, newop, score) = self.doit(&best_operations, &mut best_map);
            if best_score > score {
                best_score = score;
                apply(&mut best_map, &best_operations[ith], -1);
                apply(&mut best_map, &newop, 1);
                best_operations[ith] = newop;
            }
        }

        println_stderr!("tries: {}", tried);
        println_stderr!("score: {}", best_score);

        println!("{}", best_operations.len());
        for op in best_operations {
            println!("{} {} {}", op.x, op.y, op.h);
        }
    }


    fn left_time(&mut self) -> u64 {
        let elp = self.start_time.elapsed();
        let mi = elp.as_secs() * 1000 + elp.subsec_nanos() as u64 / 1_000_000;
        TIMEOUT_MILLIS - mi
    }

    fn generate(&mut self) -> Operation {
        Operation {
            x: self.random.next_int(N),
            y: self.random.next_int(N),
            h: self.random.next_int(N)+1
        }
    }

    fn doit(&mut self, base_operations: &Vec<Operation>, base_map: &mut TheMap) -> (usize, Operation, u64) {
        let ith = self.random.next_int(Q);
        let mut newop = self.generate();

        if self.left_time() <= 200000 {
            newop.x = self.tweak(base_operations[ith].x, 0, N - 1, 1);
            newop.y = self.tweak(base_operations[ith].y, 0, N - 1, 1);
            newop.h = self.tweak(base_operations[ith].h, 1, N, 1);
        } else {
            newop.x = self.tweak(base_operations[ith].x, 0, N - 1, 2);
            newop.y = self.tweak(base_operations[ith].y, 0, N - 1, 2);
            newop.h = self.tweak(base_operations[ith].h, 1, N, 2);
        }

        apply(base_map, &base_operations[ith], -1);
        apply(base_map, &newop, 1);

        let sc = diff(&self.map, &base_map);

        apply(base_map, &newop, -1);
        apply(base_map, &base_operations[ith], 1);

        (ith, newop, sc)
    }

    fn tweak(&mut self, v: usize, min: usize, max: usize, range: usize) -> usize {
        let diff = (self.random.next_int(range * 2 + 1)) as i32 - range as i32;
        if (v as i32) + diff < min as i32 {
            min
        } else if (v as i32) + diff > max as i32 {
            max
        } else {
            ((v as i32) + diff) as usize
        }
    }
}


fn apply(map: &mut TheMap, op: &Operation, sign: i32) {
    let cy: i32 = op.y as i32;
    let cx: i32 = op.x as i32;
    let h: i32 = op.h as i32;
    for y in cy-h..cy+h+1 {
        if y < 0 || y >= N as i32 {
            continue
        }
        for x in cx-h..cx+h+1 {
            if x < 0 || x >= N as i32 {
                continue
            }
            let diff = (cy-y).abs() + (cx-x).abs();
            if diff >= h {
                continue
            }
            map[y as usize][x as usize] += (h - diff) * sign;
        }
    }
}

fn diff(map: &TheMap, rmap: &TheMap) -> u64 {
    let mut cost: u64 = 0;
    for i in 0..N {
        for j in 0..N {
            cost += (map[i][j] - rmap[i][j]).abs() as u64;
        }
    }
    cost
}

fn score(map: &TheMap, ops: &Vec<Operation>) -> u64 {
    let mut rmap = vec![vec![0; N]; N];
    for op in ops {
        apply(&mut rmap, &op, 1);
    }

    let mut cost: u64 = 0;
    for i in 0..N {
        for j in 0..N {
            cost += (map[i][j] - rmap[i][j]).abs() as u64;
        }
    }
    cost
}

fn main() {
    let start_time = Instant::now();

    let mut random = XorShift::new();
    let mut map: TheMap = vec![vec![]; N];
    for i  in 0..N {
        map[i] = readnc();
    }
    GlobalState { start_time: start_time, random: random, map: map }.exec();
}
