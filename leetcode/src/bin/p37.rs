pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let mut board = [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
      .into_iter().map(|row| row.into_iter().map(|it| it.chars().next().unwrap()).collect()).collect();

    let expected: Vec<Vec<char>> = [["5","3","4","6","7","8","9","1","2"],["6","7","2","1","9","5","3","4","8"],["1","9","8","3","4","2","5","6","7"],["8","5","9","7","6","1","4","2","3"],["4","2","6","8","5","3","7","9","1"],["7","1","3","9","2","4","8","5","6"],["9","6","1","5","3","7","2","8","4"],["2","8","7","4","1","9","6","3","5"],["3","4","5","2","8","6","1","7","9"]]
      .into_iter().map(|row| row.into_iter().map(|it| it.chars().next().unwrap()).collect()).collect();

    Solution::solve_sudoku(&mut board);

    assert_eq!(expected, board);
  }
}

use std::collections::HashSet;

// fn print_board(board: &Vec<Vec<char>>) {
//   for row in board {
//     for item in row {
//       print!("{} ", item);
//     }
//     println!("");
//   }
//   println!("---");
// }

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
      let mut row_presences = vec![vec![false; 9]; 9];
      let mut col_presences = vec![vec![false; 9]; 9];
      let mut sq_presences = vec![vec![vec![false; 9]; 3]; 3];

      for i in 0..9 {
        for j in 0..9 {
          if board[i][j] == '.' {
            continue;
          }

          let digit = (board[i][j] as u8 - '1' as u8) as usize;
          row_presences[i][digit] = true;
          col_presences[j][digit] = true;
          sq_presences[i/3][j/3][digit] = true;
        }
      }
      let mut possibilities = vec![vec![HashSet::<usize>::with_capacity(9); 9]; 9];
      let mut to_fill = vec![];
      for i in 0..9 {
        for j in 0..9 {
          if board[i][j] != '.' {
            continue;
          }

          for c in 1..=9 {
            if !row_presences[i][c - 1] && !col_presences[j][c-1] && !sq_presences[i/3][j/3][c-1] {
              possibilities[i][j].insert(c);
            }
          }

          if possibilities[i][j].len() == 1 {
            // println!("{} {}: {}", i, j, possibilities[i][j].iter().next().unwrap());
            to_fill.push((i, j));
          }
        }
      }

      // println!("The board at the beginning:");
      // print_board(&board);

      // println!("Possibilities:");
      // for i in 0..9 {
      //   for j in 0..9 {
      //     println!("At {} {}: {:?}", i+1, j+1, possibilities[i][j]);
      //   }
      // }

      while !to_fill.is_empty() {
        let (i, j) = to_fill.pop().unwrap();
        if possibilities[i][j].len() == 0 {
          println!("{} {} had probabilities, but is now empty", i, j);
          break;
        }
        let item = *possibilities[i][j].iter().next().unwrap();
        board[i][j] = (item as u8 + '0' as u8) as char;
        possibilities[i][j].remove(&item);
        // println!("Inserting item {} at {} {}", item, i, j);
        // print_board(&board);

        // Update everything in the same row
        for col_j in 0..9 {
          if possibilities[i][col_j].contains(&item) {
            // println!("Removing {} from {},{}", item, i, col_j);
            possibilities[i][col_j].remove(&item);

            if possibilities[i][col_j].len() == 1 {
              to_fill.push((i, col_j));
            }
          }
        }

        // Update everything in the same col
        for row_i in 0..9 {
          if possibilities[row_i][j].contains(&item) {
            // println!("Removing {} from {},{}", item, row_i, j);

            possibilities[row_i][j].remove(&item);

            if possibilities[row_i][j].len() == 1 {
              to_fill.push((row_i, j));
            }
          }
        }

        // Update everything in the same sq
        for row_i in (i/3*3)..(i/3*3+3) {
          for col_j in (j/3*3)..(j/3*3+3) {
            if possibilities[row_i][col_j].contains(&item) {
              // println!("Removing {} from box: {},{}", item, row_i, j);
              possibilities[row_i][col_j].remove(&item);

              if possibilities[row_i][col_j].len() == 1{
                to_fill.push((row_i, col_j));
              }
            }
          }
        }
      }
    }
}
