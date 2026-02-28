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
    let _n = read_val_line::<usize>();
    let a = read_vector_line::<i32>();
    let b = read_vector_line::<i32>();

    let (_, indices) = a.iter().zip(b.iter())
      .map(|(a_i, b_i)| {a_i - b_i})
      .enumerate()
      .fold((i32::MIN, vec![]), |mut acc, (i, diff_i)| {
        if diff_i > acc.0 {
          return (diff_i, vec![i + 1]);
        } else if diff_i == acc.0 {
          acc.1.push(i + 1);
          return (acc.0, acc.1);
        } else {
          return acc;
        }
      });


    println!("{}", indices.len());
    for index in indices {
      print!("{} ", index);
    }
    println!();
  }
}
