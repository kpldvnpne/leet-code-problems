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

fn calc_n_ops(size: usize, before: usize, after: usize) -> usize {
  let (max, min) = if before > after { (before, after) } else { (after, before) };

  let mut n_ops = 0;
  if min > 0 {
    n_ops += 1;
  }

  let to_cover = max + size;
  let mut covered = size;
  while covered < to_cover {
    covered = covered * 2;
    n_ops += 1;
  }


  return n_ops;
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let n = read_val_line::<usize>();
    let a = read_vector_line::<u32>();

    let mut running = 1;
    let mut min_ops = usize::MAX;
    for i in 1..n+1 {
      if i < n && a[i] == a[i-1] {
        running += 1;
      } else {
        let before = i - running;
        let after = n - i;
        let n_ops = calc_n_ops(running, before, after);
        min_ops = min_ops.min(n_ops);
        running = 1;
      }
    }

    println!("{}", min_ops);
  }
}