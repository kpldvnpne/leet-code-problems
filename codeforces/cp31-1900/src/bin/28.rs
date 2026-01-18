fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

use std::fmt::Debug;
use std::str::FromStr;
fn read_val_line<T: FromStr>() -> T
  where <T as FromStr>::Err : Debug
{
  read_line().trim().parse().unwrap()
}

fn read_vector_line<T: FromStr>() -> Vec<T>
  where <T as FromStr>::Err : Debug
{
  read_line().trim().split_ascii_whitespace().map(|it| it.trim().parse().unwrap()).collect()
}

fn main() {
  let t = read_val_line::<usize>();
  for _ in 0..t {
    let [n, k] = read_vector_line::<usize>()[..2] else { panic!() };
    let a = read_vector_line::<u64>();

    let med = if n % 2 == 0 { n / 2 - 1} else { n / 2 };
    let skip = n - med;
    let mut i = a.len();
    let mut ans = 0;
    for _ in 0..k {
      i = i - skip;
      ans += a[i];
    }

    println!("{}", ans);
  }
}
