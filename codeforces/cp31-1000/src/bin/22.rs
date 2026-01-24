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

fn get_n_divs(a: u32, b: u32) -> u32 {
  let mut rem = a;
  let mut count = 0;
  while rem != 0 {
    rem = rem / b;
    count += 1;
  }

  return count;
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let [a, b] = read_vector_line::<u32>()[..2] else { panic!() };

    let mut min_ops = u32::MAX;
    let mut step = if b == 1 { 1 } else { 0 };
    loop {
      if step >= min_ops {
        break;
      }

      let new_b = b + step;
      let n_divs = get_n_divs(a, new_b);

      min_ops = std::cmp::min(min_ops, step + n_divs);
      step += 1;
    }

    println!("{}", min_ops);
  }
}
