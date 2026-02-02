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
    let b = read_vector_line::<u32>();

    let mut min_i: i32 = -1;
    let mut max_i: i32 = -1;
    for i in 0..n {
      if a[i] != b[i] {
        max_i = i as i32;
        if min_i == -1 {
          min_i = i as i32;
        }
      }
    }


    if min_i == -1 {
      // Find the largest subarray which is in non-descending order
      let mut largest = (0, 0);
      let mut l = 0;
      for i in 1..n {
        if a[i-1] > a[i] {
          l = i;
        }

        if i - l > largest.1 - largest.0 {
            largest = (l, i);
          }
      }

      println!("{} {}", largest.0 + 1, largest.1 + 1);
    } else {
      let mut min_i = min_i as usize;
      let mut max_i = max_i as usize;
      while min_i > 0 && b[min_i-1] <= b[min_i] {
        min_i -= 1;
      }

      while max_i < n - 1 && b[max_i] <= b[max_i+1] {
        max_i += 1;
      }

      println!("{} {}", min_i + 1, max_i + 1);
    }
  }
}
