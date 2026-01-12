struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test1() {
    let prices = vec![1, 7, 9, 8, 2];
    let k = 2;

    let expected = 14;
    let actual = Solution::maximum_profit(prices, k);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let prices = vec![12, 16, 19, 19, 8, 1, 19, 13, 9];
    let k = 3;

    let expected = 36;
    let actual = Solution::maximum_profit(prices, k);

    assert_eq!(expected, actual);
  }
}

impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
      let n = prices.len();
      let k = k as usize;
      let mut dp: Vec<Vec<i64>> = vec![vec![0; n]; k];

      let mut max_profit: i64 = 0;
      for i in 1..n {
        for i_k in 0..k {
          dp[i_k][i] = dp[i_k][i-1]; // When the current is not sold

          if dp[i_k][i] == 0 {
            break;
          }
        }
        for j in (0..i).rev() {
          let profit: i64 = (prices[i] - prices[j]).abs() as i64; // Normal or short-selling we dont care
          dp[0][i] = std::cmp::max(dp[0][i], profit);
          max_profit = std::cmp::max(max_profit, dp[0][i]);

          if j > 0 {
            for i_k in 0..k {
              let previous = if i_k > 0 { dp[i_k-1][j-1] } else { 0 };

              if i_k > 0 && previous == 0 {
                break;
              }

              dp[i_k][i] = std::cmp::max(previous + profit, dp[i_k][i]);
              max_profit = std::cmp::max(dp[i_k][i], max_profit);
            }
          }
        }
      }
      return max_profit;
    }
}
