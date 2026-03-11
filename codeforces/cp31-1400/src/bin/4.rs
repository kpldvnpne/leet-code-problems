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
    let [n, k] = read_vector_line::<usize>()[..2] else { panic!() };
    let mut a = read_vector_line::<u64>();

    if k < 1 || k > 2 {
      println!("0");
      continue;
    }

    a.sort();

    let mut ans = a[0];
    if k == 1 {
      for i in 1..n {
        ans = ans.min(a[i].abs_diff(a[i-1]));
      }
    } else if k == 2 {
      for i in 0..n {
        for j in i+1..n {
          let diff = a[j].abs_diff(a[i]);
          ans = ans.min(diff);

          let mut left = 0;
          let mut right = n;
          while left < right {
            let mid = (left + right) / 2;
            if diff <= a[mid] {
              right = mid;
            } else {
              left = mid + 1;
            }
          }

          let index = left;
          if index < n {
            ans = ans.min(a[index].abs_diff(diff));
          }

          if index > 0 {
            ans = ans.min(diff.abs_diff(a[index-1]))
          }
        }
      }
    }

    println!("{}", ans);
  }
}
