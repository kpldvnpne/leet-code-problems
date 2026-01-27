pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  trait ToVecOfVec<T> {
      fn to_vec_of_vec(self) -> Vec<Vec<T>>;
  }

  impl<I, J, T> ToVecOfVec<T> for I
      where I: IntoIterator<Item=J>, J: IntoIterator<Item = T>
  {
      fn to_vec_of_vec(self) -> Vec<Vec<T>> {
      self.into_iter()
          .map(|inner| inner.into_iter().collect())
          .collect()
      }
  }

  #[test]
  fn test1() {
    let board = [
      ["5","3",".",".","7",".",".",".","."]
      ,["6",".",".","1","9","5",".",".","."]
      ,[".","9","8",".",".",".",".","6","."]
      ,["8",".",".",".","6",".",".",".","3"]
      ,["4",".",".","8",".","3",".",".","1"]
      ,["7",".",".",".","2",".",".",".","6"]
      ,[".","6",".",".",".",".","2","8","."]
      ,[".",".",".","4","1","9",".",".","5"]
      ,[".",".",".",".","8",".",".","7","9"]
    ].to_vec_of_vec();

    let board = board.into_iter().map(|row| row.into_iter().map(|it| it.chars().next().unwrap()).collect()).collect();

    let expected = true;
    let actual = Solution::is_valid_sudoku(board);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let board = [
      ["8","3",".",".","7",".",".",".","."]
      ,["6",".",".","1","9","5",".",".","."]
      ,[".","9","8",".",".",".",".","6","."]
      ,["8",".",".",".","6",".",".",".","3"]
      ,["4",".",".","8",".","3",".",".","1"]
      ,["7",".",".",".","2",".",".",".","6"]
      ,[".","6",".",".",".",".","2","8","."]
      ,[".",".",".","4","1","9",".",".","5"]
      ,[".",".",".",".","8",".",".","7","9"]
    ].to_vec_of_vec();

    let board = board.into_iter().map(|row| row.into_iter().map(|it| it.chars().next().unwrap()).collect()).collect();

    let expected = false;
    let actual = Solution::is_valid_sudoku(board);

    assert_eq!(expected, actual);
  }
}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
      let mut row_present = vec![vec![false; 9]; 9];
      let mut col_present = vec![vec![false; 9]; 9];
      let mut square_present = vec![vec![vec![false; 9]; 3]; 3];

      for i in 0..9 {
        for j in 0..9 {
          let item = board[i][j];
          if item != '.' {
            let item = (item as u8 - '1' as u8) as usize;

            if row_present[i][item] {
              return false;
            } else {
              row_present[i][item] = true;
            }

            if col_present[j][item] {
              return false;
            } else {
              col_present[j][item] = true;
            }

            let square_row = i / 3;
            let square_col = j / 3;
            if square_present[square_row][square_col][item] {
              return false;
            } else {
              square_present[square_row][square_col][item] = true;
            }
          }
        }
      }

      return true;
    }
}
