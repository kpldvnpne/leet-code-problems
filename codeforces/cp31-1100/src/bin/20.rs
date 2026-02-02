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
  } else {
    return gcd(b, a % b);
  }
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let n = read_val_line::<usize>();
    let a = read_vector_line::<u64>();
    let sum = a.iter().sum::<u64>();

    let mut sum_before = 0;
    let mut max_hcf = 1;
    for item in &a[0..(n-1)] {
      sum_before += item;
      let sum_after = sum - sum_before;

      let hcf = gcd(sum_before, sum_after);
      max_hcf = max_hcf.max(hcf);
    }

    println!("{}", max_hcf);
  }
}
