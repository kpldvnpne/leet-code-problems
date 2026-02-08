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
    let a = read_vector_line::<u32>();

    if n < 4 {
      println!("-1");
      continue;
    }

    let mut min = 1;
    let mut max = n as u32;
    let mut i = 0;
    let mut j = n - 1;
    let mut is_ans = false;
    while j - i >= 3 { // As long as there is 4 elements between them
      if a[i] == min {
        i += 1;
        min = min + 1;
        continue;
      } 

      if a[i] == max {
        i += 1;
        max = max - 1;
        continue;
      }

      if a[j] == min {
        j -= 1;
        min = min + 1;
        continue;
      }

      if a[j] == max {
        j -= 1;
        max = max - 1;
        continue;
      }

      is_ans = true;
      break;
    }

    if is_ans {
      println!("{} {}", i + 1, j + 1);
    } else {
      println!("-1");
    }
  }
}