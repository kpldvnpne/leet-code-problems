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
    let _n = read_val_line::<usize>();
    let a = read_vector_line::<i64>();

    let mut min = a[0].abs();
    let mut sum = 0;
    let mut n_negs = 0;
    for item in a {
      if item < 0 {
        n_negs += 1;
      }
      min = min.min(item.abs());
      sum = sum + item.abs();
    }

    let ans = if n_negs % 2 == 0 { sum } else { sum - 2 * min };
    println!("{}", ans);
  }
}
