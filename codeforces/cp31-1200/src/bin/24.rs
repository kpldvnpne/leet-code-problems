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
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let n = read_val_line::<usize>();
    let s = read_line();
    let s = s.trim().as_bytes();

    let n_0s = s.iter().filter(|&&c| c == '0' as u8).count();

    if n_0s == 0 {
      println!("DRAW");
    } else if n % 2 == 1 && s[n / 2] == '0' as u8 {
      let n_0s = n_0s - 1;
      if n_0s == 0 {
        println!("BOB");
      } else {
        println!("ALICE");
      }
    } else {
      println!("BOB");
    }
  }
}