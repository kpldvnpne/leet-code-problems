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
    let n = read_val_line::<usize>();

    let mut min_req_powers = Vec::with_capacity(n);
    for _ in 0..n {
      let k_a = read_vector_line::<i32>();
      let k = k_a[0] as usize;
      let a = &k_a[1..k_a.len()];

      let mut min_req_power = i32::MIN;
      for (i, &a_i) in a.iter().enumerate() {
        let req_power = a_i + 1 - i as i32;
        min_req_power = min_req_power.max(req_power);
      }

      min_req_powers.push((min_req_power, k));
    }

    min_req_powers.sort_by(|a, b| { a.0.cmp(&b.0) });
    let mut min_ans = i32::MIN;
    let mut cum = 0;
    for (req_power, size) in min_req_powers {
      min_ans = min_ans.max(req_power - cum);
      cum += size as i32;
    }

    println!("{}", min_ans);
  }
}
