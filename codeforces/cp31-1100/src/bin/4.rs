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
  // println!("{} {}", 101%2 ,-99%2);
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let n = read_val_line::<usize>();
    let a = read_vector_line::<i32>();

    let mut sum = a[0];
    let mut max_sum = sum;
    for i in 1..n {
      let item = a[i];
      let prev = a[i-1];
      if (prev%2).abs() != (item%2).abs() {
        sum = sum.max(0) + item;
      } else {
        sum = item;
      }

      max_sum = max_sum.max(sum);

    }

    println!("{}", max_sum);
  }
}
