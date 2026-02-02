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
    let mut c = read_vector_line::<u64>();
    c.sort();

    if c[0] != 1 {
      println!("NO");
      continue;
    }

    let mut sum = 1;
    let mut is_possible = true;
    for i in 1..n {
      if c[i] > sum {
        is_possible = false;
        break;
      }

      sum += c[i];
    }

    if is_possible {
      println!("YES");
    } else {
      println!("NO");
    }
  }

}
