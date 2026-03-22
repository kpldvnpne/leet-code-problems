pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let mat = vec![
      vec![0, 1],
      vec![1, 0],
    ];
    let target = vec![
      vec![1, 0],
      vec![0, 1],
    ];

    let expected = true;
    let actual = Solution::find_rotation(mat, target);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let mat = vec![
      vec![0, 1],
      vec![1, 1],
    ];
    let target = vec![
      vec![1, 0],
      vec![0, 1],
    ];

    let expected = false;
    let actual = Solution::find_rotation(mat, target);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let mat = vec![
      vec![0, 0, 0],
      vec![0, 1, 0],
      vec![1, 1, 1],
    ];
    let target = vec![
      vec![1, 1, 1],
      vec![0, 1, 0],
      vec![0, 0, 0],
    ];

    let expected = true;
    let actual = Solution::find_rotation(mat, target);

    assert_eq!(expected, actual);
  }
}

fn rotate_90(x: usize, y: usize, n: usize) -> (usize, usize) {
    let mid = (n as f32 - 1.0) / 2.0;
    let x = x as f32 - mid;
    let y = y as f32 - mid;

    let (x, y) = (-y, x);
    let (x, y) = (x + mid, y + mid);

    return (x as usize, y as usize);
}

impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let n = mat.len();

        let mut exists = false;
        for rot in 0..4 {
            let mut is_possible = true;
            'outer: for i in 0..n {
                for j in 0..n {
                    let mut target_i = i;
                    let mut target_j = j;
                    for _ in 0..rot {
                        let (new_i, new_j) = rotate_90(target_i, target_j, n);
                        target_i = new_i;
                        target_j = new_j;
                    }

                    if mat[i][j] != target[target_i][target_j] {
                        is_possible = false;
                        break 'outer;
                    }
                }
            }

            if is_possible {
                exists = true;
                break;
            }
        }

        return exists;
    }
}
