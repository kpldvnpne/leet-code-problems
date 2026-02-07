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
    let [n, _k] = read_vector_line::<u32>()[..2] else { panic!() };
    let n = n as usize;
    let c = read_vector_line::<u32>();

    use std::collections::HashMap;
    let mut last_pos_by_color = HashMap::new();
    let mut max_step_by_color = HashMap::new();

    for (i, c_i) in c.into_iter().enumerate() {
      let last_pos = last_pos_by_color.entry(c_i).or_insert(0);
      let new_pos = i + 1;
      let step: usize = if *last_pos == 0 { new_pos - 1 } else { new_pos - *last_pos - 1 };
      *last_pos = new_pos;
      let last2_max_steps = max_step_by_color.entry(c_i).or_insert(vec![]);
      last2_max_steps.push(step);
      last2_max_steps.sort_by(|a, b| b.cmp(&a));
      if last2_max_steps.len() > 2 {
        last2_max_steps.pop();
      }
    }

    for (c, last_pos) in last_pos_by_color {
      let step = n - last_pos;
      let last2_max_steps = max_step_by_color.get_mut(&c).unwrap();
      last2_max_steps.push(step);
      last2_max_steps.sort_by(|a, b| b.cmp(&a));
      if last2_max_steps.len() > 2 {
        last2_max_steps.pop();
      }
    }

    let mut min = n;
    for v in max_step_by_color.values() {
      let [max, second_max] = v[..2] else { panic!() };
      let max_with_repainting = second_max.max(max / 2);
      min = min.min(max_with_repainting);
    }

    let ans = min;
    println!("{}", ans);
  
  }
}