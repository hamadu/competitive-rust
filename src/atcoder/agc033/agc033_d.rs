// https://atcoder.jp/contests/agc033/tasks/agc033_d
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

    let mut width = vec![vec![vec![vec![0; w+1]; h]; h]; 2];
    let mut height = vec![vec![vec![vec![0; h+1]; w]; w]; 2];
    for i in 0..h {
        for j in 0..h {
            for k in 0..w+1 {
                width[0][i][j][k] = k;
                width[1][i][j][k] = k;
            }
        }
    }
    for i in 0..w {
        for j in 0..w {
            for k in 0..h+1 {
                height[0][i][j][k] = k;
                height[1][i][j][k] = k;
            }
        }
    }

    for i in 0..h {
        let mut maxw = 0;
        let mut k = 0;
        while k < w {
            while maxw < w && table[i][k] == table[i][maxw] {
                maxw += 1;
            }
            while k < maxw {
                width[0][i][i][k] = maxw;
                k += 1;
            }
        }
    }
    for i in 0..w {
        let mut maxh = 0;
        let mut k = 0;
        while k < h {
            while maxh < h && table[k][i] == table[maxh][i] {
                maxh += 1;
            }
            while k < maxh {
                height[0][i][i][k] = maxh;
                k += 1;
            }
        }
    }

    for i in 0..h {
        for k in 0..w {
            let mut hasw = table[i][k] == '.';
            let mut hasb = table[i][k] == '#';
            let mut best_length = width[0][i][i][k];
            for j in i+1..h {
                hasw |= table[j][k] == '.';
                hasb |= table[j][k] == '#';
                best_length = min(best_length, width[0][j][j][k]);
                if hasw && hasb {
                    break;
                }
                width[0][i][j][k] = best_length;
            }
        }
    }

    for i in 0..w {
        for k in 0..h {
            let mut hasw = table[k][i] == '.';
            let mut hasb = table[k][i] == '#';
            let mut best_length = height[0][i][i][k];
            for j in i+1..w {
                hasw |= table[k][j] == '.';
                hasb |= table[k][j] == '#';
                best_length = min(best_length, height[0][j][j][k]);
                if hasw && hasb {
                    break;
                }
                height[0][i][j][k] = best_length;
            }
        }
    }

    let mut t2 = vec![vec![0; w+1]; h+1];
    for i in 0..h {
        for j in 0..w {
            t2[i+1][j+1] = t2[i+1][j] + t2[i][j+1] - t2[i][j];
            if table[i][j] == '#' {
                t2[i+1][j+1] += 1;
            }
        }
    }

    let mut cur = 0;
    loop {
        let fr = cur % 2;
        let to = 1 - fr;
        if width[fr][0][h-1][0] == w || height[fr][0][w-1][0] == h {
            break;
        }

        for i in 0..h {
            for j in i..h {
                for k in 0..w {
                    let v = width[fr][i][j][k];
                    width[to][i][j][k] = max(width[to][i][j][k], width[fr][i][j][v]);
                    if width[to][i][j][k] > k {
                        let mut ki = width[to][i][j][k]-1;
                        height[to][k][ki][i] = max(height[to][k][ki][i], j+1);
                    }
                }
            }
        }

        for i in 0..w {
            for j in i..w {
                for k in 0..h {
                    let v = height[fr][i][j][k];
                    height[to][i][j][k] = max(height[to][i][j][k], height[fr][i][j][v]);
                    if  height[to][i][j][k] > k {
                        let mut ki = height[to][i][j][k]-1;
                        width[to][k][ki][i] = max(width[to][k][ki][i], j+1);
                    }
                }
            }
        }

        for i in 0..w {
            for k in 0..h {
                let mut ki = height[to][i][w-1][k];
                for j in (i..w).rev() {
                    height[to][i][j][k] = max(height[to][i][j][k], ki);
                    ki = max(ki, height[to][i][j][k]);
                }
            }
        }

        for i in 0..h {
            for k in 0..w {
                let mut ki = width[to][i][h-1][k];
                for j in (i..h).rev() {
                    width[to][i][j][k] = max(width[to][i][j][k], ki);
                    ki = max(ki, width[to][i][j][k]);
                }
            }
        }
        cur += 1;
    }

    println!("{}", cur);
}
