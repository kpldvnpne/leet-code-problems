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
  let t = read_val_line::<usize>();
  for _ in 0..t {
    let [a, b] = read_vector_line::<u64>()[..2] else { panic!() };
    if a == b {
      println!("0 0");
      continue;
    }

    use std::cmp::{max, min};
    let min_val = min(a, b);
    let max_val = max(a, b);
    let max_gcd = max_val - min_val;
    let min_ops = min(min_val % max_gcd, max_gcd - min_val % max_gcd);

    println!("{max_gcd} {min_ops}");
  }
}
