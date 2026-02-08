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

fn dir(a: u32, b: u32) -> i32 {
  let diff = a as i32 - b as i32;
  if diff == 0 {
    return 0;
  }
  return diff / diff.abs();
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let n = read_val_line::<usize>();
    let a = read_vector_line::<u32>();

    if n == 1 {
      println!("1");
      continue;
    }

    // let mut b = vec![];
    // b.push(a[0]);
    let mut prev_dir = dir(a[1], a[0]);
    let mut b_size = if prev_dir == 0 { 0 } else { 1 };
    for i in 2..n {
      if a[i] == a[i-1] {
        continue;
      }
      let new_dir = dir(a[i], a[i-1]);
      if new_dir != prev_dir {
        // b.push(a[i-1]);
        b_size += 1;
      }
      prev_dir = new_dir;
    }
    b_size += 1;
    // b.push(a[n-1]);

    println!("{}", b_size);
  }
}