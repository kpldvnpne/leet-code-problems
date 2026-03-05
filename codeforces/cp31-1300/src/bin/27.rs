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
  let [n, h] = read_vector_line::<u32>()[..2] else { panic!() };
  let a = read_vector_line::<u32>();

  let mut list = Vec::with_capacity(n as usize);
  let mut ans = 1;
  for (i, &a_i) in a.iter().enumerate() {
    list.push(a_i);
    list.sort();

    // println!("{:?}", list);

    let mut iter = list.iter().rev();
    let mut height = 0;
    let mut possible = true;
    while let Some(item) = iter.next() {
      iter.next();

      // println!("{}", item);

      height += item;

      if height > h {
        possible = false;
      }
    }

    if possible {
      ans = i + 1;
    } else {
      break;
    }
  }

  println!("{}", ans);
}
