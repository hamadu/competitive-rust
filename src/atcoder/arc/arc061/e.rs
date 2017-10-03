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

const INF: i32 = 100000000;

fn main() {
    let (n, m): (usize, usize) = read();
    let edges: Vec<(usize, usize, usize)> = readn(m);
    let mut station_companies: Vec<HashSet<usize>> = vec![HashSet::new(); n];

    let mut code_map = HashMap::new();
    for &(p, q, c) in &edges {
        station_companies[p-1].insert(c);
        station_companies[q-1].insert(c);

        let code = station_code(p-1, c);
        if !code_map.contains_key(&code) {
            let len = code_map.len();
            code_map.insert(code, n + len);
        }

        let code = station_code(q-1, c);
        if !code_map.contains_key(&code) {
            let len = code_map.len();
            code_map.insert(code, n + len);
        }
    }

    let vn = code_map.len() + n;
    let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; vn];

    // between station
    for &(p, q, c) in &edges {
        let u = find_code(p-1, c, &code_map);
        let v = find_code(q-1, c, &code_map);
        graph[u].push((v, 0));
        graph[v].push((u, 0));
    }

    // terminal and station
    for u in 0..n {
        for &cid in &station_companies[u] {
            let v = find_code(u, cid, &code_map);
            graph[u].push((v, 1));
            graph[v].push((u, 0));
        }
    }

    let mut dp: Vec<i32> = vec![INF; vn];
    dp[0] = 0;

    let mut heap = BinaryHeap::new();
    heap.push((0, 0));
    while let Some(head) = heap.pop() {
        let current_cost = -head.0;
        if dp[head.1] < current_cost {
            continue
        }
        for &(to, cost) in &graph[head.1] {
            let to_cost = current_cost + cost;
            if dp[to] > to_cost {
                dp[to] = to_cost;
                heap.push((-to_cost, to));
            }
        }
    }


    if dp[n-1] == INF {
        println!("-1");
    } else {
        println!("{}", dp[n-1]);
    }
}

fn find_code(station_id: usize, company_id: usize, map: &HashMap<usize, usize>) -> usize {
    let station = station_code(station_id, company_id);
    *map.get(&station).unwrap()
}

fn station_code(station_id: usize, company_id: usize) -> usize {
    station_id * 1000001 + company_id
}