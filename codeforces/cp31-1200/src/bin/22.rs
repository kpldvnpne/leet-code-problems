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
  let [n, k, x] = read_vector_line::<u64>()[..3] else { panic!() };
  let mut a = read_vector_line::<u64>();
  a.sort();
  let mut gaps = vec![];
  for i in 1..(n as usize) {
    let gap = a[i] - a[i-1];
    if gap > x {
      gaps.push(gap);
    }
  }
  gaps.sort();

  let mut new_k = 0;
  let mut gaps_removed = 0;
  for &gap in &gaps {
    let add_k = (gap - 1) / x;
    if new_k + add_k <= k {
      gaps_removed += 1;
      new_k = new_k + add_k;
    } else {
      break;
    }
  }

  let ans = gaps.len() - gaps_removed + 1;
  println!("{}", ans);
}
