pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  fn assert_f64_close(a: f64, b: f64, epsilon: f64) {
    assert!((a - b).abs() < epsilon, "Floats are not close enough: |{a} - {b}| >= {epsilon}");
  }

  #[test]
  fn test1() {
    let squares = vec![
      vec![0, 0, 1],
      vec![2, 2 ,1],
    ];

    let expected = 1.00000;
    let actual = Solution::separate_squares(squares);

    assert_f64_close(expected, actual, 1e-5);
  }

  #[test]
  fn test2() {
    let squares = vec![
      vec![0, 0, 2],
      vec![1, 1, 1],
    ];

    let expected = 1.00000;
    let actual = Solution::separate_squares(squares);

    assert_f64_close(expected, actual, 1e-5);
  }

  #[test]
  fn test10() {
    let squares = vec![
      vec![15, 21, 2],
      vec![19, 21, 3],
    ];

    let expected = 22.30000;
    let actual = Solution::separate_squares(squares);

    assert_f64_close(expected, actual, 1e-5);
  }

  #[test]
  fn test97() {
    let squares = vec![
      vec![26, 29, 3],
      vec![10, 24, 1],
    ];

    let expected = 30.33333;
    let actual = Solution::separate_squares(squares);

    assert_f64_close(expected, actual, 1e-5);
  }



  #[test]
  fn test143() {
    let squares = vec![vec![17,27,1],vec![9,13,1],vec![1,17,4]];

    let expected = 19.0;
    let actual = Solution::separate_squares(squares);

    assert_f64_close(expected, actual, 1e-5);
  }

  #[test]
  fn test697() {
    let squares = vec![
      vec![999892931,999974790,6788622],
      vec![319710671,963660807,5518783],
      vec![623736653,934759633,4248549],
      vec![234214719,848813522,417010],
      vec![154771654,645515409,9370045],
      vec![965571354,998982755,10809560],
      vec![338822522,550588284,12471651],
      vec![168193362,682286828,5173004],
      vec![459856474,778674604,5635628],
      vec![806653114,860720237,1444683]
    ];

    let expected = 685078297.72824;
    let actual = Solution::separate_squares(squares);

    assert_f64_close(expected, actual, 1e-5);
  }
}

use std::collections::BTreeSet;

struct IntervalTree {
  xs: Vec<i32>,
  covered: Vec<i32>,
  count: Vec<i32>,
}

impl IntervalTree {
  fn new(xs: Vec<i32>) -> Self {
    let n = xs.len();
    Self {
      xs: xs,
      covered: vec![0; n * 4],
      count: vec![0; n * 4],
    }
  }

  fn _update(&mut self, ql: i32, qr: i32, left: usize, right: usize, delta: i32, pos: usize) {
    if left == right {
      return;
    }

    let x_left = self.xs[left];
    let x_right = self.xs[right];
    if qr <= x_left || x_right <= ql {
      return;
    }

    // println!("x_left = {x_left}, x_right = {x_right}");

    if ql <= x_left && x_right <= qr {
      // println!("x_left = {x_left}, x_right = {x_right}, updating count: delta={delta}");
      self.count[pos] += delta;
    } else {
      let mid = (left + right) / 2;
      self._update(ql, qr, left, mid, delta, pos * 2 + 1);
      self._update(ql, qr, mid, right, delta, pos * 2 + 2);
    }

    if self.count[pos] >= 1 {
      self.covered[pos] = x_right - x_left;
      // println!("Covering interval {} {} because count was {}", x_left, x_right, self.count[pos]);
    } else {
      if left + 1 == right {
        self.covered[pos] = 0;
      } else {
        self.covered[pos] = self.covered[pos * 2 + 1] + self.covered[pos * 2 + 2];
      }
    }
  }

  fn insert(&mut self, xl: i32, xr: i32, delta: i32) {
    let n = self.xs.len();
    self._update(xl, xr, 0, n-1, delta, 0)
  }

  fn current_width(&self) -> i32 {
    self.covered[0]
  }
}

impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
      let mut unique_xs = BTreeSet::new();
      let mut changes = Vec::with_capacity(squares.len() * 2);
      for square in squares {
        let x = square[0];
        let y = square[1];
        let l = square[2];

        unique_xs.insert(x);
        unique_xs.insert(x + l);

        changes.push((x, x+l, y, 1));
        changes.push((x, x+l, y + l, -1));
      }

      let mut interval_tree = IntervalTree::new(unique_xs.into_iter().collect());
      changes.sort_by(|a, b| a.2.cmp(&b.2));

      // println!("{:?}", changes);

      let mut areas = Vec::with_capacity(changes.len());
      let mut cur_area: f64 = 0.0;
      let mut last_y = -1;
      for change in changes {
        let (xl, xr, y, delta) = change;
        if y != last_y && last_y != -1 {
          areas.push((last_y, cur_area, interval_tree.current_width()));
        }
        // println!("Change: {:?}", change);
        cur_area += interval_tree.current_width() as f64 * (y - last_y) as f64; // last_y being -1 does not matter as current_width = 0 at that time
        interval_tree.insert(xl, xr, delta);
        // areas.push((y, cur_area, interval_tree.current_width()));

        last_y = y;
      }

      areas.push((last_y, cur_area, interval_tree.current_width()));

      let half_area: f64 = cur_area / 2.0;
      let index = areas.partition_point(|a| a.1 < half_area) - 1;
      let y = areas[index].0;
      let area_below = areas[index].1;
      let width = areas[index].2;
      let ans = y as f64 + (half_area - area_below) / width as f64;

      return ans;
    }
}
