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

#[allow(dead_code)]
fn read_string_array() -> Vec<String> {
    read_line().trim().split_ascii_whitespace().map(|it| it.trim().to_string()).collect()
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let [x, y, k] = read_vector_line::<u64>()[..3] else { panic!() };
    let n_ops_for_stick = ((k - 1) + (x - 2)) / (x - 1);
    let rem = n_ops_for_stick * (x-1) + 1 - k;
    let n_ops_for_coal = (y * k - rem + (x-2)) / (x - 1);
    let ans = n_ops_for_stick + n_ops_for_coal + k;

    println!("{}", ans);
  }
}
