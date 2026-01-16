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
    let mut result = a[0];
    for item in &a[1..n] {
      result = result ^ *item;
    }

    if result == 0 {
      println!("0");
    } else {
      if n % 2 == 0 {
        println!("-1");
      } else {
        println!("{}", result);
      }
    }
  }
}
