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

fn main() {
  let t = read_val_line::<usize>();
  for _ in 0..t {
    let n = read_val_line::<u64>();

    let mut q = n;
    let mut has_odd_divisor = false;
    while q > 1 {
      let r = q & 1;
      q = q >> 1;

      if q > 0 && r == 1 {
        has_odd_divisor = true;
      }
    }

    if has_odd_divisor {
      println!("YES");
    } else {
      println!("NO");
    }
  }
}
