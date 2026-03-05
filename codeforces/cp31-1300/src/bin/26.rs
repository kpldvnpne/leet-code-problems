fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

use std::fmt::Debug;
use std::str::FromStr;
#[allow(dead_code)]
fn read_val_line<T: FromStr>() -> T
  where <T as FromStr>::Err : Debug
{
  read_line().trim().parse().unwrap()
}

#[allow(dead_code)]
fn read_vector_line<T: FromStr>() -> Vec<T>
  where <T as FromStr>::Err : Debug
{
  read_line().trim().split_ascii_whitespace().map(|it| it.trim().parse().unwrap()).collect()
}

fn main() {
  let s = read_line();
  let s = s.trim();
  let mut prev = ' ';
  let mut n_v: i64 = 0;
  let mut n_w = 0;
  let mut n_o = 0;
  let mut v = vec![];
  for c in s.chars() {
    if c == 'v' {
      if prev == 'o' {
        v.push((n_w, n_o));
      }
      n_v += 1;
      n_o = 0;
    } else if c == 'o' {
      let cur_n_w = n_w + (n_v - 1).max(0);
      if prev == 'v' {
        n_w = cur_n_w;
        n_v = 0;
      }
      n_o += 1;
    }
    prev = c;
  }

  n_w = n_w + (n_v - 1).max(0);

  // println!("{} {:?}", n_w, v);

  let n_w = n_w;
  let mut ans = 0;
  for (w_i, o_i) in v {
    // println!("{} {} {}", w_i, o_i, n_w - w_i);
    ans += w_i * o_i * (n_w - w_i);
  }

  println!("{}", ans);
}
