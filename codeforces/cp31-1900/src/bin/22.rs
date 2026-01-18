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
    let _n = read_val_line::<u32>();
    let a = read_vector_line::<u32>();

    let mut n_1s = 0;
    let mut n_0s: u8 = 0;
    for item in a {
      if item == 0 {
        n_0s += 1;
      } else if item == 1 {
        n_1s += 1;
      }
    }

    let ans = n_1s * (1u64 << n_0s);

    println!("{ans}");
  }
}
