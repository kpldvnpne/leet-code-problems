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

fn to_index(c: u8) -> usize {
    (c - 'A' as u8) as usize
}

fn main() {
    let n = read_val_line::<usize>();
    for _ in 0..n {
        let line = read_line();
        let [s, t] = line.trim().split_ascii_whitespace().collect::<Vec<_>>()[..2] else { panic!() };
        let s = s.as_bytes();
        let t = t.as_bytes();

        let mut s_i: i32 = s.len() as i32 - 1;
        let mut deleted = [false; 26];
        let mut possible = true;
        for t_i in (0..t.len()).rev() {
            if deleted[to_index(t[t_i])] {
                possible = false;
                break;
            }
            while s_i > -1 && s[s_i as usize] != t[t_i] {
                deleted[to_index(s[s_i as usize])] = true;
                s_i -= 1;
            }

            if s_i < 0 {
                possible = false;
                break;
            }

            s_i -= 1;
        }

        if possible {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}