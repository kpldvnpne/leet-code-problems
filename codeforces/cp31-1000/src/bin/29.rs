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

fn get_n_ops(a: u64, b: u64) -> i64 {
  if a < b {
    return get_n_ops(b, a);
  }

  if a % b != 0 {
    return -1;
  }

  let mut factor = a / b;
  let mut n_ops = 0;
  while factor & 1 == 0 {
    n_ops += 1;

    for _ in 0..3 {
      if factor & 1 == 1 {
        break
      }
      factor = factor >> 1;
    }
  }

  if factor > 1 {
    return -1;
  }

  return n_ops;
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let [a, b] = read_vector_line::<u64>()[..2] else { panic!() };

    let ans = get_n_ops(a, b);
    println!("{}", ans);
  }
}
