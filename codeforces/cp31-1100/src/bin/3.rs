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

#[allow(dead_code)]
fn read_string_array() -> Vec<String> {
    read_line().trim().split_ascii_whitespace().map(|it| it.trim().to_string()).collect()
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let n = read_val_line::<usize>();
    let a = read_vector_line::<u64>();
    let mut sorted = a.clone();
    sorted.sort();

    use std::collections::BTreeMap;
    let mut sum_upto = BTreeMap::new();
    let mut sum = 0;
    for (i, item) in sorted.iter().enumerate() {
      sum += item;
      let entry = sum_upto.entry(item).or_default();
      *entry = (sum, i + 1);
    }

    let mut ans = Vec::with_capacity(n);
    for item in a {
      let mut n_removal;
      let mut sum = item;
      loop {
        let (_, (new_sum, n_items)) = sum_upto.range(..=sum).next_back().unwrap();

        n_removal = *n_items;
        if *new_sum == sum {
          break;
        }

        sum = *new_sum;
      }

      ans.push(n_removal - 1); // Don't count its own removal
    }

    for item in ans {
      print!("{} ", item);
    }
    println!("");
  }
}
