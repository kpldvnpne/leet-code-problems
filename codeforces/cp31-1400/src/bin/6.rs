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

    let mut next_zero = vec![vec![0; n]; 32];
    for i in 0..32 {
      let mut last_j = 0;
      for j in 0..=n {
        if j == n || (a[j] >> i) & 1 == 0 {
          while last_j < n && last_j <= j {
            next_zero[i][last_j] = j;
            last_j += 1;
          }
        }
      }
    }

    let q = read_val_line::<u32>();
    for _ in 0..q {
      let [l, k] = read_vector_line::<usize>()[..2] else { panic!() };
      let mut max = i32::MAX;
      let mut min = -1;

      let l = l - 1;
      let al = a[l];

      for i in (0..32).rev() {
        if (k >> i) & 1 == 0 {
          if (al >> i) & 1 == 1 {
            min = min.max((next_zero[i][l] - l) as i32 - 1).min(max);
          }
        } else {
          // println!("k = 1 at bit pos = {}", i);
          max = max.min((next_zero[i][l] - l) as i32 - 1);
          if max <= min {
            break;
          }
        }
        // println!("min = {}, max = {}", min, max);
      }

      // println!("min={}, max={}", min, max);

      let len = min.max(max);

      // println!("len = {}, l = {}", len, l);

      if len == -1 {
        print!("-1 ");
      } else {
        print!("{} ", len + (l+1) as i32);
      }
    }
    println!("")
  }
}
