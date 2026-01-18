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
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let a_b_c = read_vector_line::<u32>();
    let (a, b, c) = (a_b_c[0], a_b_c[1], a_b_c[2]);

    let n_first = (c / 2) + c % 2 + a;
    let n_second = c / 2 + b;

    if n_first > n_second {
      println!("First");
    } else {
      println!("Second");
    }
  }
}
