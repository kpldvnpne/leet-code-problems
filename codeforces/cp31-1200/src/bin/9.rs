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
    let a = read_vector_line::<u32>();
    
    use std::collections::HashMap;
    let mut count = HashMap::new();
    for item in a {
      let entry = count.entry(item).or_insert(0);
      *entry += 1;
    }

    let mut keys = count.keys().collect::<Vec<_>>();
    keys.sort();
    let mut prev_key = keys[0];
    let mut prev_count = count.get(prev_key).unwrap();
    let mut ans = *prev_count;
    for key in &keys[1..keys.len()] {
      let cur_count = count.get(key).unwrap();
      if **key == *prev_key + 1 {
        ans += (cur_count - prev_count).max(0);
      } else {
        ans += cur_count;
      }
      prev_key = key;
      prev_count = cur_count;
    }

    println!("{}", ans);
  }
}