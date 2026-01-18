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
  let t = read_val_line::<i32>();
  for _ in 0..t {
    let n = read_val_line::<i32>();

    let mut ans = 0;
    let mut m = n;
    while m > 9 {
      ans += 9;
      m = m / 10;
    }

    ans += m;

    println!("{ans}");
  }
}
