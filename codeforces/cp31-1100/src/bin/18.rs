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
    let [n, c] = read_vector_line::<u64>()[..2] else { panic!() };
    let n = n as usize;

    let a = read_vector_line::<u64>();

    let mut cost = a.iter().enumerate().map(|(i, item)| i as u64 + 1 + *item).collect::<Vec<_>>();
    cost.sort();

    let mut i = 0;
    let mut sum = 0;
    let mut used = 0;
    while sum <= c && i < n {
      sum += cost[i];
      if sum <= c {
        used += 1;
      }
      i = i + 1;
    }

    let ans = used;
    println!("{}", ans);
  }
}
