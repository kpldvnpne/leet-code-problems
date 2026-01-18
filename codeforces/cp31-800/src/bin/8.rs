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
  let t: i32 = read_val_line();
  for _ in 0..t {
    let n_k = read_vector_line::<i32>();
    let _ = n_k[0];
    let k = n_k[1];

    let a = read_vector_line::<i32>();
    let mut found = false;
    for item in a {
      if item == k {
        found = true;
      }
    }

    if found {
      println!("YES");
    } else {
      println!("NO");
    }
  }
}
