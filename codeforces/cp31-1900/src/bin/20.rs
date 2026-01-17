fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

use std::fmt::Debug;
use std::io::Read;
use std::str::FromStr;
fn read_val_line<T: FromStr>() -> T
  where <T as FromStr>::Err : Debug
{
  read_line().trim().parse().unwrap()
}

fn main() {
    let t = read_val_line::<usize>();
    for _ in 0..t {
        let s = read_line();
        let mut s = s.trim().as_bytes().to_owned();

        if s.len() > 1 {
            let mut s_i = 0;
            let n = s.len();
            while s_i < n && s[s_i] == s[0] {
                s_i += 1;
            }

            let mut balance = 0;
            while s_i < n {
                if s_i < n && s[s_i] == 'b' as u8 {
                    balance += 1;
                    while s_i < n && s[s_i] == 'b' as u8 {
                        s_i += 1;
                    }
                }

                if s_i < n && s[s_i] == 'a' as u8 {
                    balance -= 1;
                    while s_i < n && s[s_i] == 'a' as u8 {
                        s_i += 1;
                    }
                }
            }

            if balance == 1 {
                s[n - 1] = 'a' as u8;
            } else if balance == -1 {
                s[n - 1] = 'b' as u8;
            }

        }

        let mut new_string = String::new();
        s.as_slice().read_to_string(&mut new_string).unwrap();

        println!("{}", new_string);
    }
}