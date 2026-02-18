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
    let line = read_line();
    let [n, c] = line.trim().split_ascii_whitespace().collect::<Vec<_>>()[..2] else { panic!() };
    let n = n.parse::<usize>().unwrap();
    let c = c.chars().next().unwrap() as u8;
    let s = read_line();
    let s = s.trim().as_bytes();

    let all_c = s.iter().all(|s_i| *s_i == c);
    if all_c {
      println!("0");
      continue;
    }

    let mut found = false;
    for i in n/2+1..=n {
      if found && !all_c {
        break;
      }
      if s[i-1] == c {
        found = true;
        println!("1");
        println!("{}", i);
        break;
      }
    }

    if !found {
      println!("{}", 2);
      println!("{} {}", n-1, n);
    }
  }
}
