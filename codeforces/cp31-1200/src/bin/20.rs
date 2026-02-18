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

    let mut min = i32::MAX;
    for c in 0..26 {
      let c = 'a' as u8 + c;
      let mut i = 0;
      let mut j = n;
      let mut palin_possible = true;
      let mut n_del = 0;
      while i < j {
        if s[i] == s[j-1] {
          i = i + 1;
          j = j - 1;
          continue;
        }
        if s[i] == c {
          i = i + 1;
          n_del += 1;
        } else if s[j-1] == c {
          j = j - 1;
          n_del += 1;
        } else {
          palin_possible = false;
          break;
        }
      }

      if palin_possible {
        min = min.min(n_del);
      }
    }

    if min == i32::MAX {
      println!("-1");
    } else {
      println!("{}", min);
    }
  }
}
