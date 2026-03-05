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
  let n = read_val_line::<usize>();
  let a = read_vector_line::<u32>();
  let b = read_vector_line::<u32>();

  let mut init_positions = vec![0; n+1];
  for (i, &a_i) in a.iter().enumerate() {
    init_positions[a_i as usize] = i + 1;
  }

  let mut min_prev = init_positions[b[n-1] as usize];
  let mut ans = 0;
  for i in (0..n-1).rev() {
    let b_i = b[i];
    let init_position = init_positions[b_i as usize];
    if init_position > min_prev {
      ans += 1;
    }

    min_prev = min_prev.min(init_position);
  }

  println!("{}", ans);
}