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
    let [n, m] = read_vector_line::<usize>()[..2] else { panic!() };
    let mut biggest_before = vec![-1; n];
    for _ in 0..m {
      let [xi, yi] = read_vector_line::<usize>()[..2] else { panic!() };
      let xi = xi - 1;
      let yi = yi - 1;

      let min = xi.min(yi);
      let max = xi.max(yi);

      biggest_before[max] = biggest_before[max].max(min as i32);
    }

    let mut last = 0;
    let mut ans: i64 = 0;
    for cur in 0..n {
      let new_last = biggest_before[cur] + 1;
      last = last.max(new_last);

      ans += (cur as i32 - last + 1) as i64;
    }

    println!("{}", ans);
  }
}
