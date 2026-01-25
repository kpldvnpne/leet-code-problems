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
    let [n, _m] = read_vector_line::<usize>()[..2] else { panic!() };

    let mut n_negs = 0;
    let mut min_item = u32::MAX;
    let mut total: i32 = 0;
    for _ in 0..n {
      let a = read_vector_line::<i32>();
      for item in a {
        if item < 0 {
          n_negs += 1;
        }

        min_item = min_item.min(item.abs() as u32);
        total = total + item.abs();
      }
    }

    if n_negs % 2 == 1 {
      total -= 2 * min_item as i32;
    }

    println!("{}", total);
  }
}
