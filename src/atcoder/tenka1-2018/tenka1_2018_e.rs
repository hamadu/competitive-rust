// https://atcoder.jp/contests/tenka1-2018/tasks/tenka1_2018_e
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



fn main() {
    input! {
        h: usize, w: usize,
        table: [chars; h]
    };
    let h = h as i32;
    let w = w as i32;
    let check = |y: i32, x: i32| {
        if y < 0 || x < 0 || y >= h || x >= w {
            false
        } else {
            table[y as usize][x as usize] == '#'
        }
    };
    let mut total = 0i64;
    for i in 0..w {
        for j in i+1..w {
            let d = j-i;
            if d % 2 != 0 {
                continue;
            }
            let d = (j-i)/2;
            let c = (i+j)/2;
            for y in 0..h {
                if !check(y,i) || !check(y,j) {
                    continue;
                }
                if y >= d && check(y-d, c) {
                    total += 1;
                }
                if check(y+d, c) {
                    total += 1;
                }
            }
        }
    }

    for i in 0..h {
        for j in i+1..h {
            let d = j-i;
            if d % 2 != 0 {
                continue;
            }
            let d = (j-i)/2;
            let c = (i+j)/2;
            for x in 0..w {
                if !check(i,x) || !check(j,x) {
                    continue;
                }
                if x >= d && check(c, x-d) {
                    total += 1;
                }
                if check(c, x+d) {
                    total += 1;
                }
            }
        }
    }

    let mut sum_rightup = dvec!(0; (h+w-1) as usize, (w+1) as usize);
    for i in 0..h+w-1 {
        let ti = i as usize;
        for x in 0..w {
            let tx = x as usize;
            sum_rightup[ti][tx+1] = sum_rightup[ti][tx] + ifv!(check(i-x, x), 1, 0);
        }
    }
    let mut sum_rightdown = dvec!(0; (h+w-1) as usize, (w+1) as usize);
    for i in 0..h+w-1 {
        let ti = i as usize;
        for x in 0..w {
            let tx = x as usize;
            sum_rightdown[ti][tx+1] = sum_rightdown[ti][tx] + ifv!(check(i+x-w+1, x), 1, 0);
        }
    }

    let range_up = |gy: i32, gx: i32, fx: i32, tx: i32| {
        assert!(fx <= tx);
        let lane = gy + gx;
        if lane < 0 || lane >= h+w-1 {
            return 0;
        }
        let lane = lane as usize;
        if tx <= 0 || fx >= w {
            return 0;
        }
        let fx = max(0, fx) as usize;
        let tx = min(w, tx) as usize;
        sum_rightup[lane][tx] - sum_rightup[lane][fx]
    };

    let range_down = |gy: i32, gx: i32, fx: i32, tx: i32| {
        assert!(fx <= tx);
        let lane = w - 1 - (gx - gy);
        if lane < 0 || lane >= h+w-1 {
            return 0;
        }
        let lane = lane as usize;
        if tx <= 0 || fx >= w {
            return 0;
        }
        let fx = max(0, fx) as usize;
        let tx = min(w, tx) as usize;
        sum_rightdown[lane][tx] - sum_rightdown[lane][fx]
    };


    for i in 0..h {
        for j in 0..w {
            if !check(i, j) {
                continue;
            }

            for d in 2..h+w {
                if !check(i-d, j+d) {
                    continue;
                }
                // [fx, tx)
                let fx = j-d+1;
                let tx = j;
                total += range_up(i-d, j-d, fx, tx);
                let fx = j+d+1;
                let tx = j+2*d;
                total += range_up(i, j+2*d, fx, tx);
            }

            for d in 2..h+w {
                if !check(i+d, j+d) {
                    continue;
                }
                // [fx, tx)
                let fx = j-d+1;
                let tx = j;
                total += range_down(i+d, j-d, fx, tx);
                let fx = j+d+1;
                let tx = j+2*d;
                total += range_down(i, j+2*d, fx, tx);
            }
        }
    }

    println!("{}", total);
}
