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
  let t = read_val_line::<i32>();
  for _ in 0..t {
    let [n, k] = read_vector_line::<i32>()[..2] else { panic!{} };
    let n = n as usize;
    let s = read_line();
    let s = s.as_bytes();

    let mut count = [0; 26];
    for i in 0..n {
      let c = s[i] - 'a' as u8;
      count[c as usize] += 1;
    }

    let mut min_delete = -1;
    for i in 0..26 {
      if count[i] % 2 != 0 {
        min_delete += 1;
      }
    }

    if min_delete <= k {
      println!("YES");
    } else {
      println!("NO");
    }
  }
}
