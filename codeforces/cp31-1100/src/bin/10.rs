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

fn is_possible(x: u32, a: Vec<u32>, b: Vec<u32>, c: Vec<u32>) -> bool {
  if x == 0 {
    return true;
  }

  let x_bar = !x;
  let mut knowledge = 0;
  for arr in [a, b, c] {
    for item in arr {
      if item & x_bar != 0 {
        break;
      }
      knowledge = knowledge | item;

      if knowledge == x {
        return true;
      }
    }
  }

  return false;
}


fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let [_n, x] = read_vector_line::<u32>()[..2] else { panic!() };
    let a = read_vector_line::<u32>();
    let b = read_vector_line::<u32>();
    let c = read_vector_line::<u32>();

    if is_possible(x, a, b, c) {
      println!("YES");
    } else {
      println!("NO");
    }
  }
}
