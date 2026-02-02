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

fn gcd(a: u32, b: u32) -> u32 {
  if b > a {
    return gcd(b, a);
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
  // println!("{}", gcd(7, 31));
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let n = read_val_line::<usize>();
    let a = read_vector_line::<u32>();

    let mut hcf = 0;
    for (i, j) in (0..n).zip((0..n).rev()) {
      if i == j {
        break;
      }

      let diff = if a[i] > a[j] { a[i] - a[j] } else { a[j] - a[i] };
      hcf = gcd(hcf, diff);
    }

    println!("{}", hcf);
  }
}
