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
    let n = read_val_line::<u64>();
    let mut prime_factor = n;
    let mut i = 2;
    while i * i <= n {
      if n % i == 0 {
        prime_factor = i;
        break;
      }
      i += 1;
    }

    let a = 1 * n / prime_factor;
    let b = (prime_factor - 1) * n / prime_factor;

    println!("{} {}", a, b);
  }
}
