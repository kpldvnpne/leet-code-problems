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

fn binary_search(cum: &Vec<u64>, x: u64, n_days: u64, mut left: usize, mut right: usize) -> usize {
  while left + 1 < right {
    let mid = (left + right) / 2;
    if x >= n_days * mid as u64 + cum[mid] {
      left = mid;
    } else {
      right = mid;
    }
  }
  return left;
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let [n, x] = read_vector_line::<usize>()[..2] else { panic!() };
    let x = x as u64;
    let mut a = read_vector_line::<u64>();
    a.sort();
    let mut cum = Vec::with_capacity(n+1);
    let mut sum = 0;
    cum.push(sum);
    for a_i in &a {
      sum += a_i;
      cum.push(sum);
    }

    let mut n_days = 0;
    let mut n_packs = 0;
    let mut right = n+1;
    loop {
      let loc = binary_search(&cum, x, n_days, 0, right);
      if loc == 0 {
        break;
      }
      // println!("loc={}, cum={}", loc, cum[loc]);
      let new_days = (x - n_days * loc as u64 - cum[loc]) / loc as u64 + 1;
      n_packs += new_days * loc as u64;
      n_days += new_days;

      // println!("new_days={} n_packs={} n_days={}", new_days, n_packs, n_days);

      right = loc;
    }

    println!("{}", n_packs);
  }
}
