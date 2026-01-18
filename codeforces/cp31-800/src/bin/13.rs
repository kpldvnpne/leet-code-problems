fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

use std::fmt::Debug;
use std::str::FromStr;
fn read_val_line<T: FromStr>() -> T
  where <T as FromStr>::Err : Debug
{
  read_line().trim().parse().unwrap()
}

fn read_vector_line<T: FromStr>() -> Vec<T>
  where <T as FromStr>::Err : Debug
{
  read_line().trim().split_ascii_whitespace().map(|it| it.trim().parse().unwrap()).collect()
}

fn main() {
  let t: i32 = read_val_line();
  for _ in 0..t {
    let n: i32 = read_val_line();
    let a = read_vector_line::<i32>();

    let mut max = a[0];
    let mut max_count = 1;
    let mut b = vec![];
    for item in &a[1..a.len()] {
      let item = *item;
      if item == max {
        max_count += 1;
      } else if item > max {
        // Push current max to b
        for _ in 0..max_count {
          b.push(max);
        }
        // Update max
        max = item;
        max_count = 1;
      } else {
        b.push(item);
      }
    }

    if max_count == n {
      println!("-1");
    } else {
      println!("{} {}", b.len(), max_count);
      for item in b {
        print!("{item} ");
      }
      println!("");

      for _ in 0..max_count {
        print!("{max} ");
      }
      println!("");
    }
  }
}
