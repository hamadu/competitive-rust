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

#[derive(Debug)]
struct PanelSet {
    color_map: HashMap<usize, usize>
}

impl PanelSet {
    fn new() -> Self {
        PanelSet { color_map: HashMap::new() }
    }

    fn modify(&mut self, colors: &Vec<usize>, diff: i32) {
        for d in 0..4 {
            let cls = vec![colors[d], colors[(d+1)%4], colors[(d+2)%4], colors[(d+3)%4]];
            let code = Self::color_code(&cls);
            let now: i32 = (*self.color_map.get(&code).unwrap_or(&0)) as i32;
            if now + diff == 0 {
                self.color_map.remove(&code);
            } else {
                self.color_map.insert(code, (now + diff) as usize);
            }
        }
    }

    fn add(&mut self, colors: &Vec<usize>) {
        self.modify(colors, 1)
    }

    fn remove(&mut self, colors: &Vec<usize>) {
        self.modify(colors, -1)
    }

    fn way(&self, colors: &Vec<usize>) -> usize {
        let code = Self::color_code(&colors);
        *self.color_map.get(&code).unwrap_or(&0)
    }

    fn color_code(colors: &Vec<usize>) -> usize {
        colors[0] * 1000000000 + colors[1] * 1000000 + colors[2] * 1000 + colors[3]
    }
}


fn main() {
    let n: usize = read();
    let mut panels: Vec<Vec<usize>> = vec![];
    for i in 0..n {
        panels.push(readnc());
    }

    let mut panel_set = PanelSet::new();
    for i in 0..n {
        panel_set.add(&panels[i]);
    }


    let mut ans = 0;
    for i in 0..n {
        panel_set.remove(&panels[i]);
        for j in i+1..n {
            panel_set.remove(&panels[j]);

            let upper = panels[i].clone();
            for d in 0..4 {
                let downer = vec![panels[j][d], panels[j][(d+3)%4], panels[j][(d+2)%4], panels[j][(d+1)%4]];

                let mut ways = 1;
                let mut required_colors = vec![];
                for w in 0..4 {
                    required_colors.push(vec![upper[w], downer[w], downer[(w+1)%4], upper[(w+1)%4]]);
                }

                for req in &required_colors {
                    ways *= panel_set.way(&req);
                    panel_set.remove(&req);
                }
                ans += ways;
                for req in &required_colors {
                    panel_set.add(&req);
                }
            }
            panel_set.add(&panels[j]);
        }
    }

    println!("{}", ans);
}