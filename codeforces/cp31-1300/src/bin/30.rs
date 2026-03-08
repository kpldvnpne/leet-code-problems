fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

use std::fmt::Debug;
use std::str::FromStr;
#[allow(dead_code)]
fn read_val_line<T: FromStr>() -> T
  where <T as FromStr>::Err : Debug
{
  read_line().trim().parse().unwrap()
}

#[allow(dead_code)]
fn read_vector_line<T: FromStr>() -> Vec<T>
  where <T as FromStr>::Err : Debug
{
  read_line().trim().split_ascii_whitespace().map(|it| it.trim().parse().unwrap()).collect()
}

fn main() {
  let [n, k] = read_vector_line::<usize>()[..2] else { panic!() };
  let a = read_vector_line::<u64>();

  let mut sum = 0;
  for i in 0..k {
    sum += a[i];
  }

  let mut avgs = sum;
  let mut count = 1;
  for i in k..n {
    sum = sum - a[i-k] + a[i];
    count += 1;
    avgs += sum;
  }

  let ans: f64 = avgs as f64 / count as f64;
  println!("{}", ans);
}
