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

  #[test]
  fn test2() {
    let mut board= [[".",".","9","7","4","8",".",".","."],["7",".",".",".",".",".",".",".","."],[".","2",".","1",".","9",".",".","."],[".",".","7",".",".",".","2","4","."],[".","6","4",".","1",".","5","9","."],[".","9","8",".",".",".","3",".","."],[".",".",".","8",".","3",".","2","."],[".",".",".",".",".",".",".",".","6"],[".",".",".","2","7","5","9",".","."]]
      .into_iter().map(|row| row.into_iter().map(|it| it.chars().next().unwrap()).collect()).collect();

    let expected: Vec<Vec<char>> = [["5","1","9","7","4","8","6","3","2"],["7","8","3","6","5","2","4","1","9"],["4","2","6","1","3","9","8","7","5"],["3","5","7","9","8","6","2","4","1"],["2","6","4","3","1","7","5","9","8"],["1","9","8","5","2","4","3","6","7"],["9","7","5","8","6","3","1","2","4"],["8","3","2","4","9","1","7","5","6"],["6","4","1","2","7","5","9","8","3"]]
      .into_iter().map(|row| row.into_iter().map(|it| it.chars().next().unwrap()).collect()).collect();

    Solution::solve_sudoku(&mut board);

    assert_eq!(expected, board);
  }

  #[test]
  fn test_choose() {
    println!("{:?}", choose(5, 3));
  }
}

use std::collections::{HashSet, BinaryHeap};

fn print_board(board: &Vec<Vec<char>>) {
  for row in board {
    for item in row {
      print!("{} ", item);
    }
    println!("");
  }
  println!("---");
}

#[derive(Eq, PartialEq)]
struct ToFillItem {
  row: usize,
  col: usize,
  n_options: usize,
}

impl Ord for ToFillItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
      other.n_options.cmp(&self.n_options)
    }
}

impl PartialOrd for ToFillItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn choose_recursive(n: usize, k: usize, start_index: usize) -> Vec<Vec<usize>> {
  let mut indices = vec![];
  for i in start_index..(n - k + 1) {
    if k == 1 {
      indices.push(vec![i]);
    } else {
      for mut index in choose_recursive(n, k-1, i+1) {
        index.push(i);
        indices.push(index);
      }
    }
  }

  return indices;
}

fn choose(n: usize, k: usize) -> Vec<Vec<usize>> {
  if n < k {
    return vec![];
  }

  choose_recursive(n, k, 0).into_iter().map(|row| row.into_iter().rev().collect()).collect()
}

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
      let mut to_fill = BinaryHeap::new();
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

          to_fill.push(ToFillItem {
            row: i,
            col: j,
            n_options: possibilities[i][j].len()
          });
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
        let to_fill_item = to_fill.pop().unwrap();
        if to_fill_item.n_options > possibilities[to_fill_item.row][to_fill_item.col].len() {
          continue;
        }

        let i = to_fill_item.row;
        let j = to_fill_item.col;
        let col_indices = (0..9).map(|it| (it, j));
        let row_indices = (0..9).map(|it| (i, it));
        let cell_indices = (i/3*3..i/3*3+3).zip(j/3*3..j/3*3+3);

        if to_fill_item.n_options == 0 {
          panic!("n_options is 0");
        }

        if to_fill_item.n_options == 1 {
          let item = *possibilities[i][j].iter().next().unwrap();
          possibilities[i][j].remove(&item);

          board[i][j] = (item as u8 + '0' as u8) as char;

          let all_indices = col_indices.chain(row_indices).chain(cell_indices);

          for (i, j) in all_indices {
            if possibilities[i][j].contains(&item) {
              possibilities[i][j].remove(&item);

              to_fill.push(ToFillItem {
                row: i,
                col: j,
                n_options: possibilities[i][j].len()
              });
            }
          }
        } else {
          for indices in [col_indices.collect::<Vec<_>>(), row_indices.collect(), cell_indices.collect()] {
            let req_len = possibilities[i][j].len();
            let collected = indices.iter().filter(|(i, j)| possibilities[*i][*j].len() == req_len).collect::<Vec<_>>();

            for choices in choose(collected.len(), req_len) {
              println!("Req len: {}, Collected: {}", req_len, collected.len());
              println!("Choices: {:?}", choices);
              let first_choice = collected[choices[0]];
              let mut common = possibilities[first_choice.0][first_choice.1].clone();
              for i in &choices[1..choices.len()] {
                let choice = collected[*i];
                let possibility = &possibilities[choice.0][choice.1];
                common = common.intersection(possibility).map(|it| *it).collect();

                if common.len() != req_len {
                  break;
                }
              }

              // Can be removed from the other indices
              if common.len() == req_len {
                let choices_to_skip: HashSet<_> = choices.iter().map(|choice| collected[*choice]).collect();
                let indices_to_prune = indices.iter().filter(|it| !choices_to_skip.contains(it));
                for index in indices_to_prune {
                  let (i, j) = *index;
                  for item in &common {
                    if possibilities[i][j].contains(item) {
                      possibilities[i][j].remove(item);

                      to_fill.push(ToFillItem {
                        row: i,
                        col: j,
                        n_options: possibilities[i][j].len()
                      })
                    }
                  }
                }
              }

            }
          }
        }

      }

      println!("Board");
      print_board(&board);

      println!("Possibilities:");
      for i in 0..9 {
        for j in 0..9 {
          println!("At {} {}: {:?}", i+1, j+1, possibilities[i][j]);
        }
      }
    }
}
