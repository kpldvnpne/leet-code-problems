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

fn calculate_water(a: &Vec<u32>, h: u32) -> u64 {
  let mut ans: u64 = 0;
  for item in a {
    ans += (h as i32 - *item as i32).max(0) as u64;
  }

  ans
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let [_n, x] = read_vector_line::<usize>()[..2] else { panic!() };
    let a = read_vector_line::<u32>();

    let mut low = 0;
    let mut high = 2 * 1e9 as u32 + 1;
    while low != high {
      let mid = (low + high + 1) / 2;
      let water = calculate_water(&a, mid);
      if water <= x as u64 {
        low = mid;
      } else {
        high = mid - 1;
      }
    }

    println!("{low}");
  }
}
