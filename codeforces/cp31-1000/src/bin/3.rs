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
    let [n, p] = read_vector_line::<u32>()[..2] else { panic!() };
    let a = read_vector_line::<u32>();
    let b = read_vector_line::<u32>();

    let mut v = a.into_iter().zip(b.into_iter()).chain(vec![(n, p)]).collect::<Vec<_>>();
    v.sort_unstable_by(|a, b| a.1.cmp(&b.1));

    let mut covered = 1;
    let mut i = 0;
    let mut total_cost: u64 = p as u64;
    while covered < n {
      let (can_cover, cost) = v[i];
      let new_covered = std::cmp::min(n - covered, can_cover);
      let new_cost: u64 = cost as u64 * new_covered as u64;
      covered += new_covered;
      total_cost += new_cost;
      i += 1;
    }

    let ans = total_cost;
    println!("{ans}");
  }
}
