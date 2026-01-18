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
    let [a, b, _n] = read_vector_line::<i64>()[..3] else { panic!() };
    let x = read_vector_line::<i64>();

    let mut total_seconds = b;
    for item in x {
      total_seconds += std::cmp::min(item, a - 1);
    }

    println!("{total_seconds}");
  }
}