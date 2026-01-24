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

fn main() {
    let t = read_val_line::<u32>();
    for _ in 0..t {
        let n = read_val_line::<usize>();
        let s = read_vector_line::<u32>();

        // println!("{:?}", s);

        if n == 1 {
            println!("-1");
            continue;
        }

        let mut exists = true;
        let mut running = s[0];
        let mut running_start = 0;
        let mut segments = vec![];
        for i in 1..=n {
            if i < n && s[i] == running {
                continue;
            } else {
                if i - running_start == 1 {
                    exists = false;
                    break;
                } else {
                    segments.push((running_start + 1, i));
                    running_start = i;

                    if i < n {
                        running = s[i];
                    }
                }
            }
        }

        // println!("{:?}", segments);

        if !exists {
            println!("-1");
        } else {
            for segment in segments {
                print!("{} ", segment.1);
                for i in segment.0..segment.1 {
                    print!("{} ", i);
                }
            }
            println!("");
        }
    }
}