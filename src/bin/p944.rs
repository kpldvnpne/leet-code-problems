struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test1() {
    let strs = vec!["cba", "daf", "ghi"].iter().map(|it| it.to_string()).collect();
    let expected = 1;
    let actual = Solution::min_deletion_size(strs);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let strs = vec!["a", "b"].iter().map(|it| it.to_string()).collect();
    let expected = 0;
    let actual = Solution::min_deletion_size(strs);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let strs = vec!["zyx", "wvu", "tsr"].iter().map(|it| it.to_string()).collect();
    let expected = 3;
    let actual = Solution::min_deletion_size(strs);
    assert_eq!(expected, actual);
  }
}

impl Solution {
  pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let ascii_strs: Vec<&[u8]> = strs.iter().map(|it| it.as_bytes()).collect();

    let mut n_del_cols = 0;
    for i in 0..strs[0].len() {
      for str_i in 1..ascii_strs.len() {
        let cur_str = ascii_strs[str_i];
        let prev_str = ascii_strs[str_i-1];
        if prev_str[i] > cur_str[i] {
          n_del_cols += 1;
          break;
        }
      }
    }

    return n_del_cols;
  }
}
