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
    let [n, _q] = read_vector_line::<usize>()[..2] else { panic!() };
    let mut a = read_vector_line::<u32>();
    let x = read_vector_line::<u32>();

    let mut max_allowed = 31;
    for item in x {
      if item > max_allowed {
        continue;
      }

      let divisor = 1 << item;
      let add = 1 << (item - 1);
      for i in 0..n {
        if a[i] % divisor == 0 {
          a[i] += add;
        }
      }

      max_allowed = item - 1;
    }

    for item in a {
      print!("{} ", item);
    }
    println!("");
  }
}
