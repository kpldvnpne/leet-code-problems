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

// 26 minutes
fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let _n = read_val_line::<usize>();
    let x = read_vector_line::<u32>();
    let y = read_vector_line::<u32>();

    let mut over_pay = vec![];
    let mut under_pay = vec![];

    for (x_i, y_i) in x.into_iter().zip(y) {
      if y_i >= x_i {
        over_pay.push(y_i - x_i);
      } else {
        under_pay.push(x_i - y_i);
      }
    }

    over_pay.sort_by(|a, b| b.cmp(&a));
    under_pay.sort_by(|a, b| b.cmp(&a));

    let mut i = 0;
    let mut j = 0;
    let mut max_days = 0;
    while i < over_pay.len() && j < under_pay.len() {
      if over_pay[i] >= under_pay[j] {
        max_days += 1;
        i = i + 1;
        j = j + 1;
      } else {
        j = j + 1;
      }
    }

    max_days += (over_pay.len() - i) / 2;

    println!("{}", max_days);
  }
}

// -3 0 -8 2 1 5
// 5 2 1 0
// -8 -3
// 