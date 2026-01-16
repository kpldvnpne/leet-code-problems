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
    let _: i32 = read_val_line();
    let a = read_vector_line::<i32>();
    let mut new_vector = Vec::new();
    new_vector.push(a[0]);
    let mut last_item = a[0];
    for item in &a[1..a.len()] {
      if last_item <= *item {
        new_vector.push(*item);
      } else {
        new_vector.push(*item);
        new_vector.push(*item);
      }

      last_item = *item;
    }

    println!("{}", new_vector.len());
    for item in new_vector {
      print!("{} ", item);
    }
    println!("");
  }

}
