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

const MAX_NUM: usize = 1_000_000;

fn main() {
  let _n = read_val_line::<usize>();
  let a = read_vector_line::<u64>();

  let mut count = vec![0; MAX_NUM + 1];
  let sum = a.iter().sum::<u64>();
  for &a_i in &a {
    count[a_i as usize] += 1;
  }

  let mut j = vec![];

  for (i, &a_i) in a.iter().enumerate() {
    count[a_i as usize] -= 1;

    let rem_sum = sum - a_i;
    if rem_sum % 2 == 0 && rem_sum / 2 <= MAX_NUM as u64 && count[(rem_sum/2) as usize] >= 1 {
      j.push(i+1);
    }

    count[a_i as usize] += 1;
  }

  println!("{}", j.len());
  for j_i in j {
    print!("{} ", j_i);
  }
  println!();
}
