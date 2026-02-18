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

// Required 19 minutes
fn main() {
  let [n, q] = read_vector_line::<usize>()[..2] else { panic!() };
  let mut a = read_vector_line::<u64>();
  let mut sum = a.iter().sum::<u64>();
  let mut last_updated: Vec<i32> = vec![0; n];
  let mut base_value = 0;
  let mut last_fresh: i32 = -1;
  for q_i in 1..=q {
    let line = read_vector_line::<u64>();
    let t = line[0];
    match t {
      1 => {
        let i = line[1] as usize - 1;
        let x = line[2];
        let last_x = if last_updated[i] < last_fresh { base_value } else { a[i] };
        sum = sum - last_x + x;
        a[i] = x;
        last_updated[i] = q_i as i32;
      },
      2 => {
        let x = line[1];
        sum = n as u64 * x;
        last_fresh = q_i as i32;
        base_value = x;
      },
      _ => unreachable!()
    };

    println!("{}", sum);
  }
}
