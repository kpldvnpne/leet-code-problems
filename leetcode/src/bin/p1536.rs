pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let grid = [[0,0,1],[1,1,0],[1,0,0]].into_iter().map(|it| it.into_iter().collect::<Vec<_>>()).collect::<Vec<_>>();
    let expected = 3;
    let actual = Solution::min_swaps(grid);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let grid = [[0,1,1,0],[0,1,1,0],[0,1,1,0],[0,1,1,0]].into_iter().map(|it| it.into_iter().collect::<Vec<_>>()).collect::<Vec<_>>();
    let expected = -1;
    let actual = Solution::min_swaps(grid);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let grid = [[1,0,0],[1,1,0],[1,1,1]].into_iter().map(|it| it.into_iter().collect::<Vec<_>>()).collect::<Vec<_>>();
    let expected = 0;
    let actual = Solution::min_swaps(grid);

    assert_eq!(expected, actual);
  }
}

impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
      let mut zero_count = grid.into_iter().map(|row| {
        let mut n_0s = 0;
        for item in row {
          if item == 1 {
            n_0s = 0;
          } else {
            n_0s += 1;
          }
        }
        return n_0s;
      }).collect::<Vec<_>>();

      // println!("{:?}", zero_count);

      let n = zero_count.len();
      let mut ans = 0;
      for i in 0..n {
        let req_len = n - 1 - i;
        // println!("{}", req_len);
        let mut prev = 0;
        let mut found = false;
        let mut n_swaps = 0;
        for j in i..n {
          let new_prev = zero_count[j];
          zero_count[j] = prev;
          if new_prev >= req_len {
            zero_count[i] = new_prev;
            found = true;
            n_swaps = j - i;
            break;
          } else {
            prev = new_prev;
          }
        }

        if !found {
          // println!("Failed at i = {}", i);
          return -1;
        }
        // println!("N swaps: {}", n_swaps);
        ans += n_swaps;
      }

      return ans as i32;
    }
}
