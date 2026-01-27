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

use std::collections::BTreeSet;
fn get_divisors(n: usize) -> Vec<usize> {
  let mut divisors = BTreeSet::new();
  divisors.insert(1);
  divisors.insert(n);
  let mut divisor = 2;
  while divisor * divisor <= n {
    if n % divisor == 0 {
      divisors.insert(divisor);
      divisors.insert(n / divisor);
    }
    divisor += 1;
  }

  divisors.into_iter().collect()
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let n = read_val_line::<usize>();
    let a = read_vector_line::<u64>();

    let mut sum_prefix = vec![0; n];

    sum_prefix[0] = a[0];
    for (i, item) in a.iter().enumerate().skip(1) {
      sum_prefix[i] = sum_prefix[i-1] + item;
    }

    let mut max_diff = 0;
    for k in get_divisors(n) {
      let mut min = sum_prefix[k - 1];
      let mut max = sum_prefix[k - 1];

      for i in (2*k-1..n).step_by(k) {
        let cur_weight = sum_prefix[i] - sum_prefix[i-k];
        min = min.min(cur_weight);
        max = max.max(cur_weight);
      }

      let abs_diff = max - min;
      max_diff = max_diff.max(abs_diff);

      // println!("{} {}", k, max_diff);
    }

    println!("{}", max_diff);
  }
}
