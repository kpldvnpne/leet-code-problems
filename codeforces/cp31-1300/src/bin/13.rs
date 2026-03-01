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

fn sum_upto(n: i64) -> i64 {
  n * (n+1) / 2
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let [k, x] = read_vector_line::<i64>()[..2] else { panic!() };

    fn calc(m: i64, k: i64) -> i64 {
      if m <= k {
        return sum_upto(m);
      } else {
        return sum_upto(k) + sum_upto(k-1) - sum_upto(k-1-(m-k));
      }
    }

    let mut left = 0;
    let mut right = 2*k-1;
    while left < right {
      let m = (left + right) / 2;
      let n_emojis = calc(m, k);

      if n_emojis < x {
        left = m + 1;
      } else {
        right = m;
      }
    }

    // assert_eq!(left, right);
    println!("{}", left);
  }
}
