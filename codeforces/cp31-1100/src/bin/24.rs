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
    let _n = read_val_line::<usize>();
    let p = read_vector_line::<u32>();
    let mut res = -1;
    for (i, p_i) in p.iter().enumerate() {
      if i as u32 != *p_i {
        if res == -1 {
          res = *p_i as i32;
        } else {
          res = res & *p_i as i32;
        }
      }
    }
    println!("{res}");
  }
}
