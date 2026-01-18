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

#[allow(dead_code)]
fn read_vector_line<T: FromStr>() -> Vec<T>
  where <T as FromStr>::Err : Debug
{
  read_line().trim().split_ascii_whitespace().map(|it| it.trim().parse().unwrap()).collect()
}

fn main() {
  let t = read_val_line::<usize>();
  for _ in 0..t {
    let s = read_line();
    let s = s.trim();

    let mut n_0s = 0;
    let mut n_1s = 0;
    for c in s.chars() {
      if c == '0' {
        n_0s += 1;
      } else {
        n_1s += 1;
      }
    }

    let mut i = 0;
    for c in s.chars() {
      if c == '0' {
        if n_1s > 0 {
          n_1s -= 1;
        } else {
          break;
        }
      } else {
        if n_0s > 0 {
          n_0s -= 1;
        } else {
          break;
        }
      }
      i += 1;
    }

    let ans = s.len() - i;

    println!("{}", ans);
  }
}
