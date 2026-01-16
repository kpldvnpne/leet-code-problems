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

fn main() {
  let t = read_val_line::<i32>();
  for _ in 0..t {
    let n = read_val_line::<usize>();
    let s = read_line();
    let s = s.trim().as_bytes();

    let mut left = 0;
    let mut right = n - 1;
    while left < right && s[left] != s[right] {
      left += 1;
      right -= 1;
    }

    let answer = std::cmp::max(0, right as i32 - left as i32 + 1);

    println!("{}", answer);
  }
}
