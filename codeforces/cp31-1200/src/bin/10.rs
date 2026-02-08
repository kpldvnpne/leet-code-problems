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
    let [_n, _q] = read_vector_line::<usize>()[..2] else { panic!() };
    let a = read_vector_line::<u32>();
    let q = read_vector_line::<u32>();

    use std::collections::BTreeMap;
    let mut max_height_by_size = BTreeMap::new();
    let mut height: u64 = 0;
    let mut max = 0;
    max_height_by_size.insert(0, 0);
    for a_i in a {
      height += a_i as u64;
      max = max.max(a_i);

      max_height_by_size.insert(max, height);
    }

    for q_i in q {
      let range = max_height_by_size.range(..=q_i);
      let max_height = range.last().unwrap().1;
      print!("{} ", max_height);
    }
    println!("");
  }
}