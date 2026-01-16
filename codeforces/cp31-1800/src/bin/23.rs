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

fn min_moves(a:i32, b: i32, c: i32, d: i32) -> i32 {
  let x_diff = c - a;
  let y_diff = d - b;
  if y_diff < 0 {
    return -1;
  }

  if x_diff > y_diff {
    return -1;
  }

  return y_diff + (y_diff - x_diff);
}

fn main() {
  let t = read_val_line::<i32>();
  for _ in 0..t {
    let [a, b, c, d] = read_vector_line::<i32>()[0..4] else { panic!() };

    println!("{}", min_moves(a, b, c, d));
  }
}
