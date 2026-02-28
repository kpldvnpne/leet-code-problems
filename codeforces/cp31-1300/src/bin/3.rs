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
    let _n = read_val_line::<usize>();
    let a = read_vector_line::<i64>();

    use std::collections::HashSet;
    let mut prefix_sums = HashSet::new();
    let mut prefix_sum = 0;
    prefix_sums.insert(0);
    let mut found = false;
    for (i, &a_i) in a.iter().enumerate() {
      prefix_sum += if i % 2 == 0 { -a_i } else { a_i };
      if prefix_sums.contains(&prefix_sum) {
        println!("YES");
        found = true;
        break;
      } else {
        prefix_sums.insert(prefix_sum);
      }
    }

    if !found {
      println!("NO");
    }

  }
}
