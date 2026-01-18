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
        let n = read_val_line::<usize>();
        let a = read_vector_line::<i32>();

        use std::collections::HashMap;
        let mut counts = HashMap::with_capacity(n);

        let mut max_count = 0;
        for item in a {
            let count = counts.entry(item).or_insert(0u32);
            *count = *count + 1;
            max_count = std::cmp::max(max_count, *count);
        }

        let mut ans = 0;
        let mut count = max_count;
        while count < n as u32 {
            ans += 1;
            ans += std::cmp::min(n as u32 - count, count);
            count = count * 2;
        }

        println!("{ans}");
    }
}