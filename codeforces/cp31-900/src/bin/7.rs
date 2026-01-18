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
    let t = read_val_line::<i32>();
    for _ in 0..t {
        let [n, k] = read_vector_line::<i32>()[..2] else { panic!() };
        let mut a = read_vector_line::<i32>();
        a.sort();

        let mut prev_start: i32 = -1;
        let mut max_len: i32 = 0;
        for i in 0..(n as usize-1) {
            if a[i+1] - a[i] > k {
                let len = i as i32 - prev_start;
                max_len = std::cmp::max(max_len, len);
                prev_start = i as i32;
            }
        }
        let last_len = n - 1 - prev_start;
        max_len = std::cmp::max(last_len, max_len);

        let ans = n - max_len;

        println!("{ans}");
    }
}