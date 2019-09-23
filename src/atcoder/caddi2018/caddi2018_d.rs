// https://atcoder.jp/contests/caddi2018/tasks/caddi2018_d
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

const MOD: i64 = 998244353;

fn pow(a: i64, p: i64) -> i64 {
    let mut p = p;
    let mut aa = a;
    let mut ret = 1;
    while p >= 1 {
        if p & 1 == 1 {
            ret *= aa;
            ret %= MOD;
        }
        p >>= 1;
        aa = aa * aa % MOD;
    }
    ret
}

struct Board {
    n: usize,
    cells: Vec<(usize,usize,i32)>,
    data: HashMap<(usize, usize),i32>
}

impl Board {
    fn new(n: usize, m: usize, filled: Vec<(usize,usize,i32)>) -> Self {
        let mut data = HashMap::new();
        for (x,y,d) in filled.clone() {
            data.insert((x,y), d);
        }
        Board { n: n, cells: filled, data: data }
    }

    fn get(&self, x: usize, y: usize) -> i32 {
        assert!(y < self.n);
        assert!(x < self.n);
        *self.data.get(&(x, y)).unwrap_or(&(-1))
    }

    fn can(&self, x: usize, y: usize, want: i32) -> bool {
        let g = self.get(x, y);
        g == -1 || g == want
    }
}

fn solve2(board: &Board) -> (Vec<i64>, i64) {
    let mut ret = vec![0; 16];
    let mut total = 0;
    let dx = vec![0, 0, 1, 1];
    let dy = vec![0, 1, 1, 0];
    for p in 0..16 as usize {
        if p.count_ones() % 2 == 1 {
            continue;
        }
        let mut ok = true;
        let p32 = p as i32;
        for d in 0..4 {
            if !board.can(dx[d], dy[d], (p32>>d)&1) {
                ok = false;
                break;
            }
        }
        if ok {
            ret[p] = 1;
            total += 1;
        }
    }
    (ret, total)
}

fn solve3(board: &Board) -> i64 {
    let (first, total) = solve2(board);
    if board.n <= 2 {
        return total;
    }

    let dx = vec![0, 1, 2, 2, 2];
    let dy = vec![2, 2, 2, 1, 0];
    let mut dp = dvec!(0i64; 2, 16);
    for i in 0..16 {
        dp[0][i] = first[i];
    }

    let mut even16 = vec![];
    let mut even32 = vec![];
    let mut even32_o = vec![];
    for p in 0..32i32 {
        if p.count_ones() % 2 != 0 {
            continue;
        }
        if p < 16 {
            even16.push(p);
        }
        if p & 1 == p >> 4 & 1 {
            even32.push(p);
        } else {
            even32_o.push(p);
        }
    }

    for i in 2..board.n {
        let fr = i % 2;
        let to = 1 - fr;
        let x = i-2;
        let y = i-2;
        fill!(dp[to], 0);
        for &p in &even16 {
            let base = dp[fr][p as usize];
            if base == 0 {
                continue;
            }
            let w = if (p>>2) & 1 == 0 { &even32 } else { &even32_o };
            for &z in w {
                let mut ok = true;
                for d in 0..5 {
                    if !board.can(x+dx[d], y+dy[d], (z>>d)&1) {
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    continue;
                }
                let tp = ((p>>2)&1)|(((z>>1)&7)<<1);
                let tp = tp as usize;
                if tp.count_ones() % 2 == 0 {
                    dp[to][tp] += base;
                    dp[to][tp] %= MOD;
                }
            }
        }
    }

    let mut ret = 0;
    for i in 0..16i32 {
        if i.count_ones() % 2 == 0 {
            ret += dp[board.n%2][i as usize];
        }
    }
    ret % MOD
}

fn solve_outer(board: &Board) -> i64 {
    if board.n <= 3 {
        1
    } else {
        let mut filled = 0i64;
        let mut ans = 1;
        for &(x, y, w) in &board.cells {
            if max(x,y) - min(x,y) >= 3 {
                filled += 1;
                let counterpart = board.get(y, x);
                if counterpart == -1 {
                    // ok
                } else if counterpart == w {
                    // ok, count half of them
                    if x < y {
                        filled -= 1;
                    }
                } else {
                    // ng
                    ans = 0;
                    break;
                }

                if board.can(y, x, w) {

                }
            }
        }
        let f3 = (board.n - 3) as i64;
        let total = f3 * (f3 + 1) / 2 - filled;
        ans * pow(2, total) % MOD
    }
}

fn main() {
    input! {
        n: usize, m: usize,
        filled: [(usize1, usize1, i32); m]
    };

    let board = Board::new(n, m, filled);
    let inner = solve3(&board);
    let outer = solve_outer(&board);
    // debug!(inner, outer);

    println!("{}", inner * outer % MOD)
}
