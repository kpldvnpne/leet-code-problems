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

fn main() {
  let t = read_val_line::<usize>();
  for _ in 0..t {
    let [_n, x] = read_vector_line::<u64>()[..2] else { panic!() };
    let a = read_vector_line::<u64>();

    let mut max_beauty: u64 = 0;
    let mut total_sum: u64 = 0;
    for item in a {
      max_beauty += (item + x - 1) / x;
      total_sum += item;
    }

    let min_beauty = (total_sum + x - 1) / x;

    println!("{} {}", min_beauty, max_beauty);
  }
}
