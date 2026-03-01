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
    let [_n, _m] = read_vector_line::<usize>()[..2] else { panic!() };
    let mut k = read_vector_line::<usize>();
    let c = read_vector_line::<u32>();

    k.sort_by(|a, b| b.cmp(&a));

    let mut i = 0;
    let mut total_cost: u64 = 0;
    for k_i in k {
      if i < k_i {
        total_cost += c[i] as u64;
        i += 1;
      } else {
        total_cost += c[k_i - 1] as u64;
      }
    }

    println!("{}", total_cost);
  }
}
