fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

use std::fmt::Debug;
use std::str::FromStr;
fn read_val_line<T: FromStr>() -> T
  where <T as FromStr>::Err : Debug
{
  read_line().trim().parse().unwrap()
}

fn read_vector_line<T: FromStr>() -> Vec<T>
  where <T as FromStr>::Err : Debug
{
  read_line().trim().split_ascii_whitespace().map(|it| it.trim().parse().unwrap()).collect()
}

fn find_repeats(x: &[u8], s: &[u8], start_i: usize) -> i32 {
  for i in 0..s.len() {
    if x[(i + start_i) % x.len()] != s[i] {
      return -1;
    }
  }

  return ((s.len() + start_i + x.len() - 1) / x.len()) as i32;
}

fn ceil_log2(a: i32) -> i32 {
  let mut ans = 1;
  let mut log2 = 0;
  while ans < a {
    ans = ans * 2;
    log2 += 1;
  }
  return log2;
}

fn main() {
  let t: i32 = read_val_line();
  for _ in 0..t {
    let n_m: Vec<i32> = read_vector_line::<i32>();
    let _ = n_m[0];
    let _ = n_m[1];

    let x = read_line();
    let x = x.trim().as_bytes();
    let s = read_line();
    let s = s.trim().as_bytes();

    let mut n_repeats = -1;
    for start_i in 0..x.len() {
      n_repeats = find_repeats(x, s, start_i);
      if n_repeats != -1 {
        break;
      }
    }
    if n_repeats != -1 {
      println!("{}", ceil_log2(n_repeats));
    } else {
      println!("-1");
    }
  }
}
