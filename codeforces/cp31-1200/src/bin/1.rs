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

fn get_top3(a: &Vec<u32>) -> Vec<(usize, u32)> {
  let mut max_3 = vec![];
  for (i, a_i) in a.iter().enumerate() {
    let a_i = *a_i;
    max_3.push((i, a_i));
    max_3.sort_by(|a, b| b.1.cmp(&a.1));
    if max_3.len() > 3 {
      max_3.pop();
    }
  }
  return max_3;
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let _n = read_val_line::<usize>();
    let a= read_vector_line::<u32>();
    let b = read_vector_line::<u32>();
    let c = read_vector_line::<u32>();

    let a_max3 = get_top3(&a);
    let b_max3 = get_top3(&b);
    let c_max3 = get_top3(&c);

    // let ans = a_max3.iter().chain(b_max_3)
    //   .filter(|(i_a, _), (i_b, _)| i_a != i_b)
    //   .chain(c_max3)
    //   .filter(|((i_a, _), (i_b, _)), (i_c, _)| i_a != i_c && i_b != i_c)
    //   .map(|((_, item_a), (_, item_b)), (_, item_c)| item_a + item_b + item_c)
    //   .max();

    let mut max = u32::MIN;
    for (i_a, item_a) in &a_max3 {
      for (i_b, item_b) in &b_max3 {
        if i_a == i_b {
          continue;
        }
        for (i_c, item_c) in &c_max3 {
          if i_a == i_c || i_b == i_c {
            continue;
          }

          max = max.max(item_a + item_b + item_c);
        }
      }
    }

    println!("{}", max);
  }
}