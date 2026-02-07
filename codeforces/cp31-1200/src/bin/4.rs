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
    let mut b = read_vector_line::<i32>();
    b.sort();

    let mut skip = n - 1;
    let mut i = 0;
    let end = b.len();
    // let mut a = Vec::with_capacity(n);
    while i < end {
      print!("{} ", b[i]);
      // a.push(b[i]);
      i = i + skip;
      skip = skip - 1;
    }
    // a.push(b[b.len() - 1]);
    println!("{}", b[b.len() - 1])
  }
}