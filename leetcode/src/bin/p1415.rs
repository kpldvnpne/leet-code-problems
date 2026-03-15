pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let n = 10;
    for k in 1..=100 {
      println!("{}", Solution::get_happy_string(n, k));
    }
  }

  #[test]
  fn test2() {
    let n = 3;
    for k in 1..=12 {
      println!("{}", Solution::get_happy_string(n, k));
    }
  }
}

fn two_pow(n: i32) -> i32 {
    1 << n
}

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let max_k = 3 * two_pow(n-1);
        if k > max_k {
            return "".to_string();
        }
        let k = k - 1;

        let mut ans = String::new();
        let mut prev_rem = 4; // Does not matter at first
        for i in 1..=n {
            let modulo = if i == 1 { 3 } else { 2 };
            let divider = two_pow(n - i);
            let mut rem = (k / divider) % modulo;
            if rem >= prev_rem {
              rem += 1;
            }
            let c = (rem as u8 + 'a' as u8) as char;
            ans.push(c);
            prev_rem = rem;
        }
        return ans;
    }
}
