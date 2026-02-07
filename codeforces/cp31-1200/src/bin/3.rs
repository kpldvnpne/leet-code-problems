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

fn gcd(a: u64, b: u64) -> u64 {
  if a == 0 {
    return b;
  }

  if b == 0 {
    return a;
  }

  if a % b == 0 {
    return b;
  }

  return gcd(b, a % b);
}

fn lcm(a: u64, b: u64) -> u64 {
  (a as u64 * b as u64) / gcd(a, b) as u64
}

fn sum_to_n(n: u64) -> u64 {
  (n as u64 * (n as u64 + 1)) / 2
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let [n, x, y] = read_vector_line::<u64>()[..3] else { panic!() };

    let lcm = lcm(x, y);

    let n_x = n / x - n / lcm;
    let n_y = n / y - n / lcm;

    let max_n_x = sum_to_n(n) - sum_to_n(n - n_x);
    let min_n_y = sum_to_n(n_y);

    let ans = max_n_x as i64 - min_n_y as i64;
    println!("{}", ans);
  }
}