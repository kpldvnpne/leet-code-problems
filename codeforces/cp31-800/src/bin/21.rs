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

fn main() {
  let t = read_val_line::<i32>();
  for _ in 0..t {
    let n = read_val_line::<usize>();
    let a = read_vector_line::<i32>();
    let mut blank_len = if a[0] == 0 { 1 } else { 0 };
    let mut max_blank_len = blank_len;
    for i in 1..n {
      if a[i] == 0 {
        if a[i-1] == 0 {
          blank_len += 1;
        } else {
          blank_len = 1;
        }
        max_blank_len = std::cmp::max(blank_len, max_blank_len);
      }
    }

    println!("{}", max_blank_len);
  }
}
