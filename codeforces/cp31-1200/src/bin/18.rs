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
    let a = read_vector_line::<i32>();
    let y = a[n-2];
    let z = a[n-1];
    if y > z {
      println!("-1");
      continue;
    }

    if z >= 0 {
      println!("{}", n-2);
      for i in 0..n-2 {
        println!("{} {} {}", i+1, n-1, n);
      }
    } else {
      let mut is_valid = true;
      for i in (0..n-2).rev() {
        if a[i] > a[i+1] {
          is_valid = false;
          break;
        }
      }
      if !is_valid {
        println!("-1");
      } else {
        println!("0");
      }
    }
  }
}
