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

fn can_make_good(arr: &Vec<i32>) -> bool {
  let mut first_item = None;
  let mut first_item_count = 0;
  let mut second_item = None;
  let mut second_item_count = 0;
  for item in arr {
    if first_item.is_none() || first_item.unwrap() == item {
      first_item = Some(item);
      first_item_count += 1;
    } else if second_item.is_none() || second_item.unwrap() == item {
      second_item = Some(item);
      second_item_count += 1;
    } else {
      return false;
    }
  }

  return first_item_count == second_item_count || first_item_count == second_item_count + 1 || first_item_count + 1 == second_item_count || second_item_count == 0;
}

fn main() {
  let t: i32 = read_val_line();
  for _ in 0..t {
    let _: i32 = read_val_line();
    let a = read_vector_line::<i32>();

    if can_make_good(&a) {
      println!("YES");
    } else {
      println!("NO");
    }
  }
}
