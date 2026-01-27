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

#[allow(dead_code)]
fn read_string_array() -> Vec<String> {
    read_line().trim().split_ascii_whitespace().map(|it| it.trim().to_string()).collect()
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let [n, k] = read_vector_line::<usize>()[..2] else { panic!() };
    let a = read_vector_line::<u32>();
    let b = read_vector_line::<u32>();

    let mut sum_a = 0;
    let mut max_b = 0;
    let mut max_exp = 0;
    for i in 0..k.min(n) {
      sum_a = sum_a + a[i];
      max_b = max_b.max(b[i]);

      max_exp = max_exp.max(sum_a + (k-1-i) as u32 * max_b);
    }

    println!("{}", max_exp);
  }
}
