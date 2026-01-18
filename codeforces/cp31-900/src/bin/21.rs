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

    let mut cur = n;
    let mut found_5 = false;
    let mut found_0 = false;
    let mut n_remove = 0;
    while cur != 0 {
      let digit = cur % 10;
      match digit {
        5 => {
          if found_0 {
            if found_5 {
              n_remove += 1;
            }
            break;
          }
          if !found_5 {
            found_5 = true;
          } else {
            n_remove += 1;
          }
        },
        0 => {
          if found_0 {
            if found_5 {
              n_remove += 1;
            }
            break;
          } else {
            found_0 = true;
          }
        },
        2 | 7 => {
          if found_5 {
            if found_0 {
              n_remove += 1;
            }
            break;
          } else {
            n_remove += 1;
          }
        },
        _ => {
          n_remove += 1;
        }
      }
      cur = cur / 10;
    }

    println!("{n_remove}");
  }
}
