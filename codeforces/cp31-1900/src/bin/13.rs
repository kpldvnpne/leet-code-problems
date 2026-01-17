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
    let t = read_val_line::<usize>();
    for _ in 0..t {
        let n = read_val_line::<i32>();
        let a= read_vector_line::<u32>();

        let mut start: i32 = 0;
        let mut end: i32 = n-1;
        while start < n && a[start as usize] == 0  {
            start = start + 1;
        }

        while start <= end && a[end as usize] == 0 {
            end = end - 1;
        }

        let ans: u32;
        if end < start {
            // All zeros
            ans = 0;
        } else {
            let mut zero_present = false;
            for i in (start)..=(end) {
                if a[i as usize] == 0 {
                    zero_present = true;
                    break;
                }
            }

            if zero_present {
                ans = 2;
            } else {
                ans = 1
            }
        }

        println!("{ans}");
    }
}