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

use std::collections::{HashMap, HashSet};
fn create_max_count(a: &Vec<u32>, n: usize) -> HashMap<u32, u32> {
  let mut a_count = HashMap::new();
  let mut roll_count = 1;
  for i in 1..n {
    if a[i] == a[i-1] {
      roll_count = roll_count + 1;
    } else {
      let count = a_count.entry(a[i-1]).or_insert(0);
      *count = std::cmp::max(*count, roll_count);
      roll_count = 1;
    }
  }
  let count = a_count.entry(a[n-1]).or_insert(0);
  *count = std::cmp::max(*count, roll_count);

  a_count
}

fn main() {
  let t = read_val_line::<usize>();
  for _ in 0..t {
    let n = read_val_line::<usize>();
    let a = read_vector_line::<u32>();
    let b = read_vector_line::<u32>();

    let a_count = create_max_count(&a, n);
    let b_count = create_max_count(&b, n);

    let mut max_len = 0u32;
    let a_keys = a_count.keys().collect::<HashSet<_>>();
    let b_keys = b_count.keys().collect::<HashSet<_>>();
    let all_keys = a_keys.union(&b_keys);
    for key in all_keys {
      max_len = std::cmp::max(a_count.get(key).unwrap_or(&0) + b_count.get(key).unwrap_or(&0), max_len);
    }

    println!("{max_len}");
  }
}
