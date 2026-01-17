pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test1() {
    let bottom_left: Vec<Vec<i32>> =  [[1,1],[2,2],[3,1]]
      .into_iter().map(|v| v.into_iter().collect()).collect();
    let top_right = [[3,3],[4,4],[6,6]]
      .into_iter().map(|v| v.into_iter().collect()).collect();
    let expected = 1;
    let actual = Solution::largest_square_area(bottom_left, top_right);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let bottom_left: Vec<Vec<i32>> =  [[1,1],[1,3],[1,5]]
      .into_iter().map(|v| v.into_iter().collect()).collect();
    let top_right = [[5,5],[5,7],[5,9]]
      .into_iter().map(|v| v.into_iter().collect()).collect();
    let expected = 4;
    let actual = Solution::largest_square_area(bottom_left, top_right);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let bottom_left: Vec<Vec<i32>> =  [[1,1],[2,2],[1,2]]
      .into_iter().map(|v| v.into_iter().collect()).collect();
    let top_right = [[3,3],[4,4],[3,4]]
      .into_iter().map(|v| v.into_iter().collect()).collect();
    let expected = 1;
    let actual = Solution::largest_square_area(bottom_left, top_right);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test4() {
    let bottom_left: Vec<Vec<i32>> =  [[1,1],[3,3],[3,1]]
      .into_iter().map(|v| v.into_iter().collect()).collect();
    let top_right = [[2,2],[4,4],[4,2]]
      .into_iter().map(|v| v.into_iter().collect()).collect();
    let expected = 0;
    let actual = Solution::largest_square_area(bottom_left, top_right);

    assert_eq!(expected, actual);
  }
}

struct Rectangle {
  bottom_left: Vec<i32>,
  top_right: Vec<i32>,
}

impl Rectangle {
  fn intersection(&self, other: &Rectangle) -> Rectangle {
    let bottom_left_x = std::cmp::max(self.bottom_left.x(), other.bottom_left.x());
    let bottom_left_y = std::cmp::max(self.bottom_left.y(), other.bottom_left.y());
    let top_right_x = std::cmp::min(self.top_right.x(), other.top_right.x());
    let top_right_y = std::cmp::min(self.top_right.y(), other.top_right.y());
    Rectangle {
      bottom_left: vec![bottom_left_x, bottom_left_y],
      top_right: vec![top_right_x, top_right_y],
    }
  }

  fn largest_square(&self) -> i64 {
    let x_diff = self.top_right.x() - self.bottom_left.x();
    let y_diff = self.top_right.y() - self.bottom_left.y();
    let min = std::cmp::min(x_diff, y_diff);
    if min < 0 {
      return 0;
    } else {
      return min as i64 * min as i64;
    }
  }
}

trait Point {
  fn x(&self) -> i32;
  fn y(&self) -> i32;
}

impl Point for Vec<i32> {
  fn x(&self) -> i32 {
    self[0]
  }

  fn y(&self) -> i32 {
    self[1]
  }
}

use std::cmp::{max, min};

impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
      let mut max_side: i32 = 0; // Which means no intersection
      let n = bottom_left.len();
      for i in 0..(n-1) {
        for j in (i+1)..n {
          let w = min(top_right[i][0], top_right[j][0]) - max(bottom_left[i][0], bottom_left[j][0]);
          let h = min(top_right[i][1], top_right[j][1]) - max(bottom_left[i][1], bottom_left[j][1]);

          let square_len = min(w, h);
          max_side = max(max_side, square_len)
        }
      }

      return max_side as i64 * max_side as i64;
    }

    pub fn largest_square_area_v1(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
      let mut rectangles = bottom_left.into_iter().zip(top_right.into_iter())
        .map(|(a, b)| Rectangle {bottom_left: a, top_right :b})
        .collect::<Vec<_>>();
      rectangles.sort_by(|a, b| a.bottom_left.x().cmp(&b.bottom_left.x()));

      let n = rectangles.len();
      let mut max_area = 0; // Which means no intersection
      for i in 0..(n-1) {
        let a = &rectangles[i];
        for j in (i+1)..n {
          let b = &rectangles[j];
          if a.top_right.x() <= b.bottom_left.x() {
            // No intersection anymore
            break;
          }

          let intersection = a.intersection(b);
          let area = intersection.largest_square();

          max_area = std::cmp::max(max_area, area);
        }
      }

      return max_area;
    }
}
