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

use std::collections::HashMap;

fn main() {
  let [_n, _q] = read_vector_line::<usize>()[..2] else { panic!() };
  let a = read_vector_line::<u32>();
  let t = read_vector_line::<u32>();

  let mut min_idxs = HashMap::new();
  for (index, item) in a.iter().enumerate() {
    min_idxs.entry(*item).or_insert(index);
  }

  for query in t {
    let min_idx = *min_idxs.get(&query).unwrap();
    print!("{} ", min_idx + 1);
    for (key, value) in min_idxs.iter_mut() {
      if *key == query {
        *value = 0;
      } else if *value < min_idx {
        *value = *value + 1;
      }
    }
  }
}