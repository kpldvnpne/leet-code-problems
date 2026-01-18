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
  let t: i32 = read_val_line();
  for _ in 0..t {
    let n: i32 = read_val_line();
    if n % 3 == 0 {
      println!("Second");
    } else {
      println!("First");
    }
  }
}
