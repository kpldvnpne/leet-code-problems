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
    let a = read_vector_line::<u32>();

    let mut arr = a.iter().enumerate().collect::<Vec<_>>();
    arr.sort_by(|a, b| b.1.cmp(&a.1));

    let mut positions = vec![0; n+1];
    let mut pos: i32 = 1;
    let mut min_time: u64 = 0;
    for item in arr {
      min_time += *item.1 as u64 * 2 * pos.abs() as u64;
      positions[item.0 + 1] = pos;
      if pos > 0 {
        pos = -pos;
      } else {
        pos = -pos + 1;
      }
    }

    println!("{}", min_time);
    for pos in positions {
      print!("{} ", pos);
    }
    println!("");
  }
}
