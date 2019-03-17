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

fn print_answer(table: Vec<Vec<usize>>) {
  let n = table.len();
  let mut num_map = vec![-1; 4*n+10];
  let mut num_index = 1;

  println!("{}", n);
  for i in 0..n {
      for j in 0..n {
          if j >= 1 {
              print!(" ");
          }
          if num_map[table[i][j]] == -1 {
            num_map[table[i][j]] = num_index;
            num_index += 1;
          }
          print!("{}", num_map[table[i][j]]);
      }
      println!("");
  }
}

fn main() {
  input! {
    w: usize
  };

  if w <= 8 {
    let mut ans = vec![vec![0; w]; w];
    for i in 0..w {
      for j in 0..w {
        ans[i][j] = j;
      }
    }
    print_answer(ans);
    return
  }

  let n = (w+1) / 2;
  let n = if n % 2 == 1 { n + 1 } else { n };
  let mut ans = vec![vec![0; n]; n];
  for i in 0..n {
    for j in 0..n {
      let offset = (i/2)*2;
      ans[i][j] = (i%2)*n+(offset+j)%n;
      if w < 2*n && ans[i][j] == n*2-1 {
        ans[i][j] = ans[i-1][(j+1)%n]; 
      }
      if w < 2*n-1 && ans[i][j] == n*2-3 {
        ans[i][j] = ans[i-1][(j+1)%n];
      }
      if w < 2*n-2 && ans[i][j] == n*2-5 {
        ans[i][j] = ans[i-1][(j+1)%n];
      }
    }
  }

  print_answer(ans);
}
