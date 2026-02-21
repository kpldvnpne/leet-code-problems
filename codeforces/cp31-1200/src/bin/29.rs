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
    let [n, k] = read_vector_line::<u32>()[..2] else { panic!() };
    let ans;
    if n % 2 == 0 {
      ans = (k - 1) % n + 1;
    } else {
      let collision_period = n / 2;
      let extra_moves = (k - 1) / collision_period;
      let location = (k + extra_moves - 1) % n + 1;
      ans = location;
    }

    println!("{}", ans);
  }
}