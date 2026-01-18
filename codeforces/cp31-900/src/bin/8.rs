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

fn main() {
    let t = read_val_line::<usize>();
    for _ in 0..t {
        let n = read_val_line::<usize>();
        let s = read_line();
        let s = s.trim().as_bytes();

        let mut max_run = 1;
        let mut run = 1;
        for i in 1..n {
            if s[i] == s[i-1] {
                run += 1;
                max_run = std::cmp::max(max_run, run);
            } else {
                run = 1;
            }
        }

        let ans = max_run + 1;

        println!("{ans}");
    }
}