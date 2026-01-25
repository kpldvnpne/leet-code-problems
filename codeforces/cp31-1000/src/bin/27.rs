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

fn get_digits(n: u64) -> Vec<u32> {
  let mut divs = vec![];
  let mut rem = n;
  while rem > 0 {
    divs.push((rem % 10) as u32);
    rem = rem / 10;
  }
  // divs.reverse();
  divs
}

// fn gcd(a: u32, b: u32) -> u32 {
//   if b % a == 0 {
//     a
//   } else {
//     gcd(b % a, a)
//   }
// }

// fn lcm(a: u32, b: u32) -> u32 {
//   a * b / gcd(a, b)
// }

fn is_fair(n: u64) -> bool {
  for digit in get_digits(n) {
    if digit != 0 && n % digit as u64 != 0 {
      return false;
    }
  }
  return true;
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let n = read_val_line::<u64>();

    let mut c = n;
    loop {
      if is_fair(c) {
        break;
      }
      c = c + 1;
    }

    println!("{}", c);
  }
}
