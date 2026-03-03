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
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let [_n, x, m] = read_vector_line::<usize>()[..3] else { panic!() };
    let mut left = x;
    let mut right = x;

    for _ in 0..m {
      let [l_i, r_i] = read_vector_line::<usize>()[..2] else {panic!()};
      let is_completely_on_left = r_i < left;
      let is_completely_on_right = right < l_i;
      let is_completely_within = left <= l_i && r_i <= right;
      if is_completely_on_left || is_completely_on_right || is_completely_within {
        // Skip
        continue;
      }

      left = left.min(l_i);
      right = right.max(r_i);
    }

    let ans = right - left + 1;
    println!("{}", ans);
  }
}