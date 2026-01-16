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

fn sum_1ton(n: i64) -> i64 {
  n * (n + 1) / 2
}

fn main() {
  let t = read_val_line::<i32>();
  for _ in 0..t {
    let [n, k, x] = read_vector_line::<i64>()[..3] else { panic!() };
    let max_possible = sum_1ton(n) - sum_1ton(n-k);
    let min_possible = sum_1ton(k);
    if min_possible <= x && x <= max_possible {
      println!("YES");
    } else {
      println!("NO");
    }
  }
}
