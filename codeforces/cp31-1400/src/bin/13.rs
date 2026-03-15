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
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let [n, k] = read_vector_line::<usize>()[..2] else { panic!() };
    let a = read_vector_line::<u32>();
    let mut greater = vec![0; n];
    for i in 1..n {
      if (a[i] << 1) > a[i-1] {
        greater[i] = 1;
      }
    }

    let mut sum = 0;
    for i in 1..k {
      sum += greater[i];
    }

    let mut ans = 0;
    for i in k..n {
      sum = sum + greater[i] - greater[i - k];
      if sum == k {
        ans += 1;
      }
    }

    println!("{}", ans);
  }
}
