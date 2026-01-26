pub struct Solution;

fn main() {}

#[allow(unused)]
fn assert_f64_close(a: f64, b: f64, epsilon: f64) {
    assert!((a - b).abs() < epsilon, "Floats are not close enough: |{a} - {b}| >= {epsilon}");
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let squares = vec![
      vec![0, 0, 1],
      vec![2, 2, 1],
    ];
    let expected = 1.00000;
    let actual = Solution::separate_squares(squares);

    assert_f64_close(expected, actual ,1e-5);
  }

  #[test]
  fn test2() {
    let squares = vec![
      vec![0, 0, 2],
      vec![1, 1, 1],
    ];
    let expected = 1.16667;
    let actual = Solution::separate_squares(squares);

    assert_f64_close(expected, actual,1e-5);
  }

  #[test]
  fn test3() {
    let squares = vec![
      vec![23, 29, 3],
      vec![28, 29, 4],
    ];
    let _ = Solution::separate_squares(squares);

    // println!("{actual}");
  }

  #[test]
  fn test429() {
    let squares = vec![
      vec![21, 27, 2],
      vec![18, 29, 3],
    ];
    let actual = Solution::separate_squares(squares);
    let expected = 29.83333;

    assert_f64_close(expected, actual, 1e-5);
  }

  #[test]
  fn test451() {
    let squares = vec![
      vec![25, 27, 1],
      vec![26, 27, 3],
    ];
    let actual = Solution::separate_squares(squares);
    let expected = 28.33333;

    assert_f64_close(expected, actual, 1e-5);
  }

  #[test]
  fn test517() {
    let squares = vec![
      vec![13, 23, 3],
      vec![12, 19, 2],
      vec![8, 18, 3],
    ];
    let actual = Solution::separate_squares(squares);
    let expected = 20.60000;

    assert_f64_close(expected, actual, 1e-5);
  }

  #[test]
  fn test909() {
    let squares = vec![
      vec![168, 187, 36],
      vec![15, 138, 17],
      vec![132, 198, 86],
      vec![93, 197, 14],
    ];
    let actual = Solution::separate_squares(squares);
    let expected = 230.64535;

    assert_f64_close(expected, actual, 1e-5);
  }
}

#[derive(Eq, PartialEq)]
struct ExitingSquare {
  y: i32, /* Last y */
  l: i64,
}

impl PartialOrd for ExitingSquare {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
      other.y.partial_cmp(&self.y) // Reverse order
    }
}

impl Ord for ExitingSquare {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

use std::collections::BinaryHeap;
impl Solution {
    pub fn separate_squares(mut squares: Vec<Vec<i32>>) -> f64 {
      squares.sort_by(|a, b| a[1].cmp(&b[1]));

      // println!("{:?}", squares);

      let mut min_y = i32::MAX;
      let mut max_y = i32::MIN;
      let mut total_area: i64 = 0;
      for square in &squares {
        let (_, y, l) = (square[0], square[1], square[2] as i64);
        min_y = std::cmp::min(y, min_y);
        max_y = std::cmp::max(y + l as i32, max_y);
        total_area += l * l;
      }

      // println!("min_y={min_y}, max_y={max_y}");

      let mut exiting_squares: BinaryHeap<ExitingSquare> = BinaryHeap::new();
      let mut cache = vec![];
      let mut s_i = 0;
      let n = squares.len();
      let mut area_below: i64 = 0;
      let mut area_increasing: i64 = 0;
      let mut prev_y = squares[0][1];
      while s_i < n {
        let square = &squares[s_i];
        let y = square[1];

        while let Some(exit_square) = exiting_squares.peek() && exit_square.y <= y {
          let exiting_y = exit_square.y;
          while let Some(exit_square) = exiting_squares.peek() && exit_square.y == exiting_y {
            area_below += area_increasing * (exit_square.y - prev_y) as i64;
            // println!("Area below = {area_below}");
            prev_y = exit_square.y;

            area_increasing -= exit_square.l;
            exiting_squares.pop();
          }

          cache.push((exiting_y, area_below, area_increasing));
        }

        area_below += (y - prev_y) as i64 * area_increasing;
        prev_y = y;

        while s_i < n && squares[s_i][1] == y {
          area_increasing += squares[s_i][2] as i64;
          exiting_squares.push(ExitingSquare { y: squares[s_i][1] + squares[s_i][2], l: squares[s_i][2] as i64 });

          s_i += 1;
        }

        if !cache.is_empty() && cache[cache.len() - 1].0 == y {
          let last_index = cache.len() - 1;
          cache[last_index].1 = area_below;
          cache[last_index].2 = area_increasing;
        } else {
          cache.push((y, area_below, area_increasing));
        }

      }

      while let Some(exit_square) = exiting_squares.peek() {
        let exiting_y = exit_square.y;
        while let Some(exit_square) = exiting_squares.peek() && exit_square.y == exiting_y {
          area_below += area_increasing * (exit_square.y - prev_y) as i64;
          prev_y = exit_square.y;
          area_increasing -= exit_square.l;
          exiting_squares.pop();
        }

        cache.push((exiting_y, area_below, area_increasing));
      }

      // println!("Cache: {:?}", cache);

      let mut top: f64 = max_y as f64;
      let mut bottom: f64 = min_y as f64;
      let half_area: f64 = total_area as f64 / 2.0;
      while !((top - bottom) < 1e-5) {
        let y = (top + bottom) / 2.0;

        // println!("Searching at y = {y}");

        // Find area at y
        let search_result = cache.binary_search_by(|it| (it.0 as f64).total_cmp(&y));
        let cache_item = match search_result {
          Ok(index) => cache[index],
          Err(index) => if index > 0 {
            cache[index-1]
          } else {
            println!("Search reasult y = {}", cache[index].0);
            panic!("Find should find at index other than 0");
          }
        };

        let area_under_y  = cache_item.1 as f64 + cache_item.2 as f64 * (y - cache_item.0 as f64);
        // println!("Area under y = {area_under_y}, Half Area = {half_area}, top = {top}, bottom = {bottom}, y = {y}");
        if area_under_y < half_area{
          bottom = y;
        } else {
          top = y;
        }
      }

      return top;
    }
}
