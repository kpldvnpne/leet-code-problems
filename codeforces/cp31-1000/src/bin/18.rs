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
    let [w, h] = read_vector_line::<u64>()[..2] else { panic!() };

    let mut max_area: u64 = 0;
    for _ in 0..2 {
      let k_x = read_vector_line::<u64>();
      let k = k_x[0] as usize;
      let x = &k_x[1..];

      max_area = std::cmp::max((x[k-1] - x[0]) * h ,max_area);
    }

    for _ in 0..2 {
      let k_y = read_vector_line::<u64>();
      let k = k_y[0] as usize;
      let y = &k_y[1..];

      max_area = std::cmp::max((y[k-1] - y[0]) * w ,max_area);
    }

    println!("{}", max_area);
  }
}
