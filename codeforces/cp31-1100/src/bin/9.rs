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
    let s = read_vector_line::<u64>();

    let c = s.iter().map(|it| *it * *it).sum::<u64>() as i128 - c as i128;
    let b = 4 * s.into_iter().sum::<u64>() as i128;
    let a = 4 * n as i128;

    // println!("{} {} {}", a, b, c);

    let ans = (-b + f64::sqrt((b * b - 4 * a * c) as f64) as i128) / (2 * a); // The only positive answer
    // let r2 = (-b - f64::sqrt((b * b - 4 * a * c) as f64) as i64) / 2 * a;
    println!("{}", ans);
  }
}
