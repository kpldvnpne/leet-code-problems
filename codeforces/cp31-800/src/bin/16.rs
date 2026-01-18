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
    let mut ans = i32::MAX;
    for i in 0..(n-1) {
      let d = a[i+1] - a[i];

      if d < 0 { // Already unsorted
        ans = 0;
        break;
      } else {
        ans = std::cmp::min(ans, d / 2 + 1);
      }
    }
    println!("{ans}");
  }
}
