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

fn read_vector_line<T: FromStr>() -> Vec<T>
  where <T as FromStr>::Err : Debug
{
  read_line().trim().split_ascii_whitespace().map(|it| it.trim().parse().unwrap()).collect()
}

fn main() {
  let t = read_val_line::<i32>();
  for _ in 0..t {
    let n_x = read_vector_line::<i32>();
    let n = n_x[0] as usize;
    let x= n_x[1];

    let a = read_vector_line::<i32>();
    let mut max_distance = a[0];
    for i in 0..(n-1) {
      max_distance = std::cmp::max(max_distance, a[i+1] - a[i]);
    }
    max_distance = std::cmp::max(max_distance, 2 * (x - a[n-1]));

    println!("{max_distance}");
  }
}

