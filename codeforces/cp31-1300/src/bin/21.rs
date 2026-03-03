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
    let n = read_val_line::<usize>();
    let p = read_vector_line::<i32>();
    
    let mut ans = vec![p[0]];
    for i in 1..(n-1) {
      if (p[i] - p[i-1]).signum() == (p[i+1] - p[i]).signum() {
        continue;
      } else {
        ans.push(p[i]);
      }
    }
    ans.push(p[n-1]);

    println!("{}", ans.len());
    for ans_i in ans {
      print!("{} ", ans_i);
    }
    println!();
  }
}