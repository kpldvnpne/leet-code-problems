pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test1() {
    let grid = vec![
      vec![1, 2, 3, 4],
      vec![5, 6, 7, 8],
      vec![9, 10, 11, 12],
      vec![13, 14, 15, 16],
    ];

    let x = 1;
    let y = 0;
    let k = 3;

    let expected =  [[1,2,3,4],[13,14,15,8],[9,10,11,12],[5,6,7,16]].into_iter().map(|row| row.into_iter().collect()).collect::<Vec<Vec<i32>>>();
    let actual = Solution::reverse_submatrix(grid, x, y, k);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let grid = vec![
      vec![3, 4, 2, 3],
      vec![2, 3, 4, 2],
    ];

    let x = 0;
    let y = 2;
    let k = 2;

    let expected =  [[3, 4, 4, 2], [2, 3, 2, 3]].into_iter().map(|row| row.into_iter().collect()).collect::<Vec<Vec<i32>>>();
    let actual = Solution::reverse_submatrix(grid, x, y, k);

    assert_eq!(expected, actual);
  }
}

impl Solution {
    pub fn reverse_submatrix(mut grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let x = x as usize;
        let y = y as usize;
        let k = k as usize;
        for i in 0..(k/2) {
            for j in 0..k {
                let temp = grid[x+i][y+j];
                grid[x+i][y+j] = grid[x+k-1-i][y+j];
                grid[x+k-1-i][y+j] = temp;
            }
        }

        return grid;
    }
}
