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
    let [_n, r, b] = read_vector_line::<usize>()[..3] else { panic!() };

    let buckets = b + 1;
    let each_bucket = r / buckets;
    let extra = r % buckets;

    for _ in 0..extra {
      for _ in 0..(each_bucket+1) {
        print!("R")
      }
      print!("B")
    }
    for i in extra..buckets {
      for _ in 0..each_bucket {
        print!("R")
      }

      // Print except for the last
      if i != (buckets - 1) {
        print!("B")
      }
    }
    println!("")
  }
}
