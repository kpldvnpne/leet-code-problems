pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::Solution;
  #[test]
  fn test1() {
    let nums1 = vec![2, 1, -2, 5];
    let nums2 = vec![3, 0, -6];
    let actual = Solution::max_dot_product(nums1, nums2);
    let expected = 18;
    assert_eq!(actual, expected);
  }

  #[test]
  fn test2() {
    let nums1 = vec![3, -2];
    let nums2 = vec![2, -6, 7];
    let actual = Solution::max_dot_product(nums1, nums2);
    let expected = 21;
    assert_eq!(actual, expected);
  }

  #[test]
  fn test3() {
    let nums1 = vec![-1, -1];
    let nums2 = vec![1, 1];
    let actual = Solution::max_dot_product(nums1, nums2);
    let expected = -1;
    assert_eq!(actual, expected);
  }

  #[test]
  fn test22() {
    let nums1 = vec![5, -4, -3];
    let nums2 = vec![-4, -3, 0, -4, 2];
    let actual = Solution::max_dot_product(nums1, nums2);
    let expected = 28;
    assert_eq!(actual, expected);
  }
}

impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
      let mut product_matrix = vec![vec![0; nums1.len()]; nums2.len()];
      for i in 0..nums2.len() {
        for j in 0..nums1.len() {
          product_matrix[i][j] = nums1[j]*nums2[i];
        }
      }

      let mut max_product_matrix = vec![vec![0; nums1.len()]; nums2.len()];

      let mut max = i32::MIN;
      for i in 0..nums2.len() {
        let mut row_max = i32::MIN;
        for j in 0..nums1.len() {
          let current = product_matrix[i][j];
          let mut current_max = current;
          if i != 0 {
            current_max = std::cmp::max(current_max, max_product_matrix[i-1][j]);
          }
          if j != 0 {
            current_max = std::cmp::max(current_max, max_product_matrix[i][j-1]);
          }
          if i != 0 && j != 0 {
            current_max = std::cmp::max(current_max, current + max_product_matrix[i-1][j-1]);
          }

          row_max = std::cmp::max(row_max, current_max);

          max_product_matrix[i][j] = row_max;
        }

        if row_max > max {
          max = row_max;
        }
      }

      return max;
    }
}
