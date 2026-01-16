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

    let mut n_2s = 0;
    for item in &a {
      if *item == 2 {
        n_2s += 1;
      }
    }

    if n_2s % 2 != 0 {
      println!("-1"); // Not possible
    } else {
      let required_2s = n_2s / 2;
      let mut k = 0;
      let mut found_2s = 0;
      while found_2s < required_2s {
        if a[k] == 2 {
          found_2s += 1;
        }
        k = k + 1;
      }

      let ans = std::cmp::max(1, k);
      println!("{ans}");
    }
  }
}
