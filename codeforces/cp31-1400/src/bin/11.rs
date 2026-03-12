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
    let a = read_vector_line::<i32>();
    let mut max = *a.iter().max().unwrap();

    let max = match max % 10 {
      0 => max,
      5 => max + 5,
      _ => {
        while max % 10 != 2 {
          max = max + (max % 10);
        }
        max
      }
    };

    let mut possible = true;
    for a_i in a {
      match a_i % 10 {
        0 => {
          if a_i != max {
            possible = false;
            break;
          }
        },
        5 => {
          if a_i + 5 != max {
            possible = false;
            break;
          }
        },
        _ => {
          let mut a_i = a_i;
          while a_i % 10 != 2 {
            a_i = a_i + (a_i % 10);
          }

          if (max - a_i) % 20 != 0 {
            possible = false;
            break;
          }
        }
      }
    }

    if possible {
      println!("YES");
    } else {
      println!("NO");
    }
  }
}
