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
    let _n = read_val_line::<usize>();
    let a = read_vector_line::<i32>();
    let mut n_negs = 0;
    let mut total = 0;
    for item in a {
      if item < 0 {
        n_negs += 1;
      }
      total += item;
    }

    let n_for_pos = if total < 0 { (-total + 1) / 2 } else { 0 };
    let required = if n_for_pos % 2 != n_negs % 2 { n_for_pos + 1 } else { n_for_pos };

    println!("{}", required);
  }
}
