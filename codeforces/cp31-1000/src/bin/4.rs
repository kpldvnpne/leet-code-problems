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
    let n = read_val_line::<usize>();

    let mut min = u32::MAX;
    let mut min_2nd =  u32::MAX;
    let mut sum: u64 = 0;
    for _ in 0..n {
      let _m = read_val_line::<usize>();
      let mut a = read_vector_line::<u32>();

      a.sort();
      sum += a[1] as u64;

      min = std::cmp::min(min, a[0]);
      min_2nd = std::cmp::min(min_2nd, a[1]);
    }

    sum = sum - min_2nd as u64 + min as u64;
    let ans = sum;

    println!("{ans}");
  }
}