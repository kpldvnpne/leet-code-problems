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
    let _n = read_val_line::<i32>();
    let a = read_vector_line::<i32>();

    let mut n_odds = 0;
    for item in a {
      if item % 2 == 1 {
        n_odds += 1;
      }
    }

    if n_odds % 2 == 0 {
      println!("YES");
    } else {
      println!("NO");
    }
  }

}
