pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let customers = String::from("YYNY");
    let output = 2;
    assert_eq!(Solution::best_closing_time(customers), output);
  }

  #[test]
  fn test3() {
    let customers = String::from("YYYY");
    let output = 4;
    assert_eq!(Solution::best_closing_time(customers), output);
  }
}

// Requirements
// 1. 0-indexed
// forward, penalty for no customer
// backward, penalty for customer
// earliest hour to close for min penalty

impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
      let n_hours = customers.len();
      let mut forward_penalties = vec![0; n_hours];
      let mut backward_penalties = vec![0; n_hours];

      let mut penalty = 0;
      for (i, char) in customers.char_indices() {
        if char == 'N' {
          penalty += 1;
        }
        forward_penalties[i] = penalty;
      }

      let mut penalty = 0;
      for (i, char) in customers.char_indices().rev() {
        if char == 'Y' {
          penalty += 1;
        }
        backward_penalties[i] = penalty;
      }

      let mut min_penalty = i32::MAX;
      let mut min_penalty_hour = 0;
      for hour in 0..=n_hours {
        let cur_penalty = if hour > 0 { forward_penalties[hour-1] } else {0} + if hour < n_hours {backward_penalties[hour]} else {0};
        if cur_penalty < min_penalty {
          min_penalty = cur_penalty;
          min_penalty_hour = hour;
        }

        // println!("Penalty {cur_penalty} at hour {hour}");
      }

      // println!("Min penalty of {min_penalty} at {min_penalty_hour}");

      return min_penalty_hour as i32;
    }
}
