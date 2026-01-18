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
  let t = read_val_line::<usize>();
  for _ in 0..t {
    let n = read_val_line::<usize>();
    let p = read_vector_line::<u32>();

    let mut i = 0;
    while i < n - 1 && p[i] >= p[i+1] {
      i = i + 1;
    }

    let mut j = i + 1;
    while j < n - 1 && p[j] <= p[j+1] {
      j = j + 1;
    }

    let k = j + 1;

    if k >= n {
      println!("NO");
    } else {
      println!("YES");
      println!("{} {} {}", i+1, j+1, k+1);
    }
  }
}
