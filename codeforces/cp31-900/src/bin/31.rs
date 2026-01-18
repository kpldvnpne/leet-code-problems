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
    let n = read_val_line::<u32>();

    let mut n_2s = 0;
    let mut rem = n;
    while rem % 2 == 0 {
      n_2s += 1;
      rem = rem / 2;
    }

    let mut n_3s = 0;
    while rem % 3 == 0 {
      n_3s += 1;
      rem = rem / 3;
    }

    if rem > 1 {
      println!("-1");
    } else {
      if n_2s > n_3s {
        println!("-1");
      } else {
        let n_ops = n_3s + (n_3s - n_2s);
        println!("{}", n_ops);
      }
    }
  }
}
