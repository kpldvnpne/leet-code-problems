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
    let [n, s] = read_vector_line::<usize>()[..2] else { panic!() };
    let s = s as u32;
    let a = read_vector_line::<u32>();
    let sum = a.iter().sum::<u32>();

    if s > sum {
      println!("-1");
      continue;
    }

    let mut front = vec![(0, 0)];
    let mut sum = 0;
    for (i, item) in a.iter().enumerate() {
      if *item == 1 {
        sum += 1;
        front.push((sum, i+1));
      }
    }

    let mut back = vec![(0, 0)];
    let mut sum = 0;
    for (i, item) in a.iter().enumerate().rev() {
      if *item == 1 {
        sum += 1;
        back.push((sum, n-i));
      }
    }

    let mut back_i = 0;
    let rem_sum = a.iter().sum::<u32>() - s;
    while back[back_i].0 < rem_sum {
      back_i += 1;
    }
    let mut min_removed = usize::MAX;
    for (front_sum, front_rem) in &front {
      let front_sum = *front_sum;
      if front_sum > rem_sum {
        break;
      }
      let rem = rem_sum - front_sum;
      while back_i > 0 && back[back_i-1].0 >= rem {
        back_i -= 1;
      }
      let removed = front_rem + back[back_i].1;
      min_removed = min_removed.min(removed);
    }

    println!("{}", min_removed);
  }
}