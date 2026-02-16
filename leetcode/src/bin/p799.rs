pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let poured = 1;
    let query_row = 1;
    let query_glass = 1;
    let expected = 0.00;
    let actual = Solution::champagne_tower(poured, query_row, query_glass);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let poured = 2;
    let query_row = 1;
    let query_glass = 1;
    let expected = 0.50;
    let actual = Solution::champagne_tower(poured, query_row, query_glass);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test303() {
    let poured = 1;
    let query_row = 0;
    let query_glass = 0;
    let expected = 1.000;
    let actual = Solution::champagne_tower(poured, query_row, query_glass);

    assert_eq!(expected, actual);
  }
}

fn get_overflow(poured: i32, query_row: i32, query_glass: i32, dp: &mut Vec<Vec<f64>>) -> f64 {
  if query_row < 0 {
    return poured as f64;
  }
  if query_glass < 0 || query_glass > query_row {
    return 0f64;
  }
  let query_glass = query_glass as usize;
  let query_row = query_row as usize;
  if query_row == 0 && query_glass == 0 {
    return ((poured - 1) as f64).max(0f64);
  }
  if dp[query_row][query_glass] != -1f64 {
    return dp[query_row][query_glass];
  }

  let from_left = get_overflow(poured, query_row as i32 - 1, query_glass as i32 - 1, dp) / 2f64;
  let from_right = get_overflow(poured, query_row as i32-1, query_glass as i32, dp) / 2f64;

  let total = from_left + from_right;
  dp[query_row][query_glass] = (total - 1f64).max(0f64);

  return dp[query_row][query_glass];
}

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
      let mut dp = vec![vec![-1f64; query_row as usize]; query_row as usize];
      let prev_left = get_overflow(poured, query_row - 1, query_glass - 1, & mut dp) / 2f64;
      let prev_right = get_overflow(poured, query_row-1, query_glass, &mut dp) / 2f64;

      let ans = (prev_left + prev_right).min(1f64);
      return ans;
    }

    pub fn champagne_tower_v1(poured: i32, query_row: i32, query_glass: i32) -> f64 {
      if poured == 0 {
        return 0f64;
      }

      if query_row == 0 {
        assert!(query_glass == 0);
        return 1f64;
      }

      let mut overflow = vec![(poured - 1) as f64];

      for i in 1..(query_row as usize) {
        let mut new_overflow = Vec::with_capacity(i+1);
        for j in 0..=(i as usize) {
          let prev_left = if j == 0 { 0f64 } else { overflow[j-1] / 2f64 };
          let prev_right = if j == i { 0f64 } else { overflow[j] / 2f64 };

          if prev_left + prev_right > 1f64 {
            new_overflow.push(prev_left + prev_right - 1f64);
          } else {
            new_overflow.push(0f64);
          }
        }

        overflow = new_overflow;
      }

      // Calculate for query_glass
      let prev_left = if query_glass == 0 { 0f64 } else { overflow[query_glass as usize-1] / 2f64 };
      let prev_right = if query_glass == query_row { 0f64 } else { overflow[query_glass as usize] / 2f64 };

      let ans = (prev_left + prev_right).min(1f64);

      return ans;
    }
}
