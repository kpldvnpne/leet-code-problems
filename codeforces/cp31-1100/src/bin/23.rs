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
    let [n, l, r] = read_vector_line::<u32>()[..3] else { panic!() };
    let mut possible = true;
    let mut v = Vec::with_capacity(n as usize);
    for i in 1..=n {
      let req = r / i * i;
      if l <= req && req <= r {
        v.push(req);
      } else {
        possible = false;
        break;
      }

    }

    if possible {
      println!("YES");
      for item in v {
        print!("{} ", item);
      }
      println!("");
    } else {
      println!("NO");
    }
  }
}
