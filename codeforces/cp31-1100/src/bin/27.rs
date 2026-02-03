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
    let [n, k] = read_vector_line::<u32>()[..2] else {panic!()};
    let _n = n as usize;
    let k = k as i32;
    let a = read_vector_line::<i32>();
    use std::collections::HashSet;
    let mut set = HashSet::new();
    for item in &a {
      set.insert(*item);
    }
    
    let mut possible = false;
    for item in a {
      // println!("For item: {}", item);
      let req = item - k;
      if set.contains(&req) {
        // println!("Contians {}", req);
        possible = true;
        break;
      }
      // println!("Does not contain {}", req);
    }

    if possible {
      println!("YES");
    } else {
      println!("NO")
    }
  }
}