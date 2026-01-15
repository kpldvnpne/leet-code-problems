struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test1() {
    let points = vec![
      vec![1, 1],
      vec![3, 4],
      vec![-1, 0],
    ];

    let expected = 7;
    let actual = Solution::min_time_to_visit_all_points(points);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let points = vec![
      vec![3, 2],
      vec![-2, 2],
    ];

    let expected = 5;
    let actual = Solution::min_time_to_visit_all_points(points);

    assert_eq!(expected, actual);
  }
}

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
      let mut total_seconds = 0;
      for i in 1..points.len() {
        let prev = &points[i-1];
        let cur = &points[i];
        let diff_x = i32::abs(cur[0] - prev[0]);
        let diff_y = i32::abs(cur[1] - prev[1]);
        let seconds = std::cmp::max(diff_x, diff_y);
        total_seconds += seconds;
      }

      return total_seconds;
    }
}
