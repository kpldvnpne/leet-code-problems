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

struct Rectangle<'a> {
  bottom_left: &'a Vec<i32>,
  top_right: &'a Vec<i32>,
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

impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
      let mut rectangles = bottom_left.iter().zip(top_right.iter())
        .map(|(a, b)| Rectangle {bottom_left: a, top_right :b})
        .collect::<Vec<_>>();
      rectangles.sort_by(|a, b| a.bottom_left.x().cmp(&b.bottom_left.x()));

      let n = rectangles.len();
      for i in 0..(n-1) {
        let a = &rectangles[i];
        for j in (i+1)..n {
          let b = &rectangles[j];
          if a.top_right.x <= b.bottom_left.x {
            // No intersection anymore
            break;
          }

          
        }
      }
    }
}
