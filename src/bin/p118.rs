pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn case_1() {
    assert_eq!(Solution::generate(5), vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1], vec![1, 4, 6, 4, 1]]);
  }

  #[test]
  fn case_2() {
    assert_eq!(Solution::generate(1), vec![vec![1]]);
  }
}

fn generate_nth_row(previous_row: &Vec<i32>) -> Vec<i32> {
  let mut new_row: Vec<i32> = vec![0; previous_row.len() + 1];

  for i in 0..new_row.len() {
    if i == 0 || i == (new_row.len() - 1) {
      new_row[i] = 1;
    } else {
      new_row[i] = previous_row[i] + previous_row[i-1];
    }
  }

  return new_row;
}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
      let mut previous_row: Vec<i32> = vec![1];
      let mut answer: Vec<Vec<i32>> = vec![previous_row.clone()];

      for _ in 1..num_rows {
        let row = generate_nth_row(&previous_row);
        answer.push(row.clone());
        previous_row = row;
      }

      return answer;
    }
}
