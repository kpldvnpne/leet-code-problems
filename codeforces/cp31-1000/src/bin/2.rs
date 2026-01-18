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
  let t = read_val_line::<usize>();
  for _ in 0..t {
    let [_n, k] = read_vector_line::<usize>()[..2] else { panic!() };
    let a = read_vector_line::<u32>();

    let mut min_ops = u32::MAX;
    let mut n_even = 0;
    for item in a {
      if item % 2 == 0 {
        n_even += 1;
      }
      let rem = item % k as u32;
      if rem == 0 {
        min_ops = 0;
        break;
      } else {
        let ops = k as u32 - rem;
        min_ops = std::cmp::min(ops, min_ops);
      }
    }

    let mut ans = min_ops;

    if k == 4 && ans > 0 {
      if n_even >= 2 {
        ans = 0;
      } else {
        ans = std::cmp::min(2 - n_even, min_ops);
      }
    }
    println!("{}", ans);
  }
}
