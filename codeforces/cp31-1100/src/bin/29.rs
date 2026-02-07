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

fn gcd(a: i64, b: i64) -> i64 {
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

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let n = read_val_line::<usize>();
    let a = read_vector_line::<i64>();

    // Using odd items
    let mut odd_gcd = a[0];
    for i in (2..n).step_by(2) {
      odd_gcd = gcd(odd_gcd, a[i]);
      if odd_gcd == 1 {
        break;
      }
    }

    // Check on even items
    let mut is_ans = true;
    for i in (1..n).step_by(2) {
      if a[i] % odd_gcd == 0 {
        is_ans = false;
      }
    }

    if is_ans {
      println!("{}", odd_gcd);
      continue;
    }

    // Using even items
    let mut even_gcd = a[1];
    for i in (3..n).step_by(2) {
      even_gcd = gcd(even_gcd, a[i]);
      if even_gcd == 1 {
        break;
      }
    }

    // Check using odd items
    let mut is_ans = true;
    for i in (0..n).step_by(2) {
      if a[i] % even_gcd == 0 {
        is_ans = false;
      }
    }

    if is_ans {
      println!("{}", even_gcd);
    } else {
      println!("{}", 0);
    }
  }
}
