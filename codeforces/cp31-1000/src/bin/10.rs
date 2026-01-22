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
        let n_c = read_line();
        let [n, c] = n_c.trim().split_ascii_whitespace().collect::<Vec<_>>()[..2] else {panic!()};
        let n: usize = n.parse().unwrap();
        let c = c.chars().nth(0).unwrap();

        let s = read_line();
        let s = s.trim();

        if c == 'g' {
            println!("0");
        } else {
            let mut last_r: i32 = -1;
            let mut last_y: i32 = -1;
            let mut first_g: i32 = -1;
            let mut max_r = 0;
            let mut max_y = 0;
            for (i, ch) in s.chars().enumerate() {
                match ch {
                    'g' => {
                        if first_g == -1 {
                            first_g = i as i32;
                        }
                        if last_r != -1 {
                            max_r = std::cmp::max(max_r, i - last_r as usize);
                            last_r = -1;
                        }

                        if last_y != -1 {
                            max_y = std::cmp::max(max_y, i - last_y as usize);
                            last_y = -1;
                        }
                    },
                    'r' => {
                        if last_r == -1 {
                            last_r = i as i32;
                        }
                    },
                    'y' => {
                        if last_y == -1 {
                            last_y = i as i32;
                        }
                    },
                    _ => unreachable!()
                }
            }

            if last_r != -1 {
                max_r = std::cmp::max(max_r, n - last_r as usize + first_g as usize);
            }

            if last_y != -1 {
                max_y = std::cmp::max(max_y, n - last_y as usize + first_g as usize);
            }

            println!("{}", if c == 'r' { max_r } else { max_y });
        }
    }
}