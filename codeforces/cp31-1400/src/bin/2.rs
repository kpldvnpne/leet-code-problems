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

    let mut i = 0;
    while i < n-1 && a[i] >= a[i+1] {
      i += 1;
    }

    if i == n-1 {
      println!("0");
      continue;
    }
    let mut top_2 = vec![a[i], a[i+1]];
    top_2.sort();
    let mut ans = 0;
    i = i + 2;
    while i < n {
      if a[i] <= top_2[0] {
        top_2[0] = a[i];
      } else if a[i] <= top_2[1] {
        top_2[1] = a[i];
      } else {
        top_2[0] = top_2[1];
        top_2[1] = a[i];
        ans += 1;
      }
      i += 1;
    }

    println!("{}", ans);
  }
}
