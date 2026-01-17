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
        if n < 4 || n % 2 != 0 {
            println!("-1")
        } else {
            let min = match n % 6 {
                2 => n / 6 - 1 + 2,
                4 => n / 6 + 1,
                0 => n / 6,
                _ => unreachable!()
            };
            let max = match n % 4 {
                0 => n / 4,
                2 => n / 4 - 1 + 1,
                _ => unreachable!()
            };
            println!("{} {}", min, max);
        }
    }
}