pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test1() {
    let n = 2;
    let m = 1;
    let hbars = vec![2, 3];
    let vbars = vec![2];

    let expected = 4;
    let actual = Solution::maximize_square_hole_area(n, m, hbars, vbars);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let n = 1;
    let m = 1;
    let hbars = vec![2];
    let vbars = vec![2];

    let expected = 4;
    let actual = Solution::maximize_square_hole_area(n, m, hbars, vbars);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let n = 2;
    let m = 3;
    let hbars = vec![2, 3];
    let vbars = vec![2, 4];

    let expected = 4;
    let actual = Solution::maximize_square_hole_area(n, m, hbars, vbars);
    assert_eq!(expected, actual);
  }
}

fn get_max_len(sorted_bars: &Vec<i32>) -> i32 {
  let mut prev = sorted_bars[0];
  let mut len = 2;
  let mut max_len = 2;
  for i in 1..sorted_bars.len() {
    let cur = sorted_bars[i];
    if cur == prev + 1 {
      len += 1;
      max_len = std::cmp::max(max_len, len);
    } else {
      len = 2;
    }
    prev = cur;
  }

  return max_len;
}

impl Solution {
    pub fn maximize_square_hole_area(n: i32, m: i32, mut h_bars: Vec<i32>, mut v_bars: Vec<i32>) -> i32 {
        h_bars.sort();
        v_bars.sort();

        let h_max_len = get_max_len(&h_bars);
        let v_max_len = get_max_len(&v_bars);

        let min = std::cmp::min(h_max_len, v_max_len);

        return min * min;
    }
}
