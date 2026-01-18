fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

use std::str::FromStr;
use std::fmt::Debug;
fn read_val_line<T: FromStr>() -> T
  where <T as FromStr>::Err : Debug
{
  read_line().trim().parse().unwrap()
}

fn read_val_vector<T: FromStr>() -> Vec<T>
  where <T as FromStr>::Err : Debug
{
  read_line().trim().split_ascii_whitespace().map(|it| it.trim().parse().unwrap()).collect()
}

fn main() {
  let t = read_val_line::<i32>();
  for _ in 0..t {
    let n_k = read_val_vector::<i32>();
    let n = n_k[0] as usize;
    let k = n_k[1];
    let a = read_val_vector::<i32>();

    let mut is_sorted = true;
    for i in 1..n {
      if !(a[i-1] <= a[i]) {
        is_sorted = false;
      }
    }

    if is_sorted {
      println!("YES");
    } else if n > 1 && k == 1 {
      println!("NO");
    } else {
      println!("YES");
    }
  }
}
