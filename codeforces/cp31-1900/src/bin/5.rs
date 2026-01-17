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
        let n = read_val_line::<i32>();
        let a = read_vector_line::<i32>();

        let mut result = 0;
        let mut ans = vec![];
        for item in a {
            result = result ^ item;
        }
        ans.push((1, n));

        if result != 0 {
            if n % 2 == 0 {
                ans.push((1, n));
            } else {
                ans.push((2, n));
                ans.push((1, 2));
                ans.push((1, 2));
            }
        }

        // Print the answer
        println!("{}", ans.len());
        for item in ans {
            println!("{} {}", item.0, item.1);
        }
    }
}