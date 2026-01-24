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

#[allow(dead_code)]
fn read_string_array() -> Vec<String> {
    read_line().trim().split_ascii_whitespace().map(|it| it.trim().to_string()).collect()
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let a = read_line();
    let a = a.trim().as_bytes();
    let b = read_line();
    let b = b.trim().as_bytes();

    let min_len = std::cmp::min(a.len(), b.len());
    let mut prev = vec![vec![true; b.len() + 1]; a.len() + 1];
    let mut max_com_len = 0;
    for len in 1..=min_len {
      let mut cur = vec![vec![false; b.len() + 1]; a.len() + 1];
      let mut found = false;

      for a_i in (len - 1)..a.len() {
        for b_i in (len - 1)..b.len() {
          if prev[a_i][b_i] && a[a_i] == b[b_i] {
            cur[a_i+1][b_i+1] = true;
            found = true;
          }
        }
      }

      if !found {
        break;
      }

      max_com_len = std::cmp::max(len, max_com_len);

      prev = cur;
    }

    let ans = a.len() - max_com_len + b.len() - max_com_len;

    println!("{ans}");
  }
}
