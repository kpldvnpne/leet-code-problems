// Last found path save
// Depth-first search with priority for the downward direction

pub struct Solution;

fn main() { }

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let row = 2;
        let col = 2;
        let cells = vec![
            vec![1, 1],
            vec![2, 1],
            vec![1, 2],
            vec![2, 2],
        ];

        let output = Solution::latest_day_to_cross(row, col, cells);

        assert_eq!(output, 2);
    }

    #[test]
    fn test2() {
        let row = 2;
        let col = 2;
        let cells = vec![
            vec![1, 1],
            vec![1, 2],
            vec![2, 1],
            vec![2, 2],
        ];

        let output = Solution::latest_day_to_cross(row, col, cells);

        assert_eq!(output, 1);
    }

    #[test]
    fn test3() {
        let row = 3;
        let col = 3;
        let cells = vec![
            vec![1, 2],
            vec![2, 1],
            vec![3, 3],
            vec![2, 2],
            vec![1, 1],
            vec![1, 3],
            vec![2, 3],
            vec![3, 2],
            vec![3, 1],
        ];

        let output = Solution::latest_day_to_cross(row, col, cells);

        assert_eq!(output, 3);
    }
}

#[derive(Copy,Clone,PartialEq,Eq,Hash,Debug)]
struct Position(usize, usize);

impl Position {
    fn down(&self) -> Self {
        Position(self.0 + 1, self.1)
    }

    fn up(&self) -> Self {
        Position(self.0 - 1, self.1)
    }

    fn left(&self) -> Self {
        Position(self.0, self.1 - 1)
    }

    fn right(&self) -> Self {
        Position(self.0, self.1 + 1)
    }
}

type Land<T> = Vec<Vec<T>>;
trait LandTrait<T> {
    fn at(&self, p: &Position) -> T;
    fn has_right_from(&self, p: &Position) -> bool;
    fn has_left_from(&self, p: &Position) -> bool;
    fn has_up_from(&self, p: &Position) -> bool;
    #[allow(dead_code)]
    fn has_down_from(&self, p: &Position) -> bool;
}

impl<T: Copy> LandTrait<T> for Land<T> {
    fn at(&self, p: &Position) -> T {
        self[p.0][p.1]
    }

    fn has_right_from(&self, p: &Position) -> bool {
        p.1 + 1 < self[0].len()
    }
    fn has_left_from(&self, p: &Position) -> bool {
        p.1 >= 1
    }
    fn has_up_from(&self, p: &Position) -> bool {
        p.0 >= 1
    }
    fn has_down_from(&self, p: &Position) -> bool {
        p.0 + 1 < self.len()
    }
}

type Path = Vec<Position>;

fn find_path_dfs(land: &Land<i32>, visited: &mut Vec<Vec<bool>>, position: &Position) -> Option<Path> {
    visited[position.0][position.1] = true;

    if !land.has_down_from(position) {
        return Some(vec![*position]);
    }

    let mut others = vec![];
    let down = position.down();
    if !visited.at(&down) && land.at(&down) == 0 {
        others.push(down);
    }

    if land.has_right_from(position) {
        let right = position.right();
        if !visited.at(&right) && land.at(&right) == 0{
            others.push(right);
        }
    }

    if land.has_left_from(position) {
        let left = position.left();
        if !visited.at(&left) && land.at(&left) == 0{
            others.push(left);
        }
    }

    if land.has_up_from(position) {
        let up = position.up();
        if !visited.at(&up) && land.at(&up) == 0{
            others.push(up);
        }
    }
    
    for item in others {
        if let Some(mut path) = find_path_dfs(land, visited, &item) {
            path.push(*position);
            return Some(path);
        }
    }

    return Option::None;
}

fn find_path(land: &Land<i32>) -> Option<Path> {
    let mut visited = vec![vec![false; land[0].len()]; land.len()];

    // Start from the top layer
    for start_pos in 0..land[0].len() {
        if land[0][start_pos] == 0 {
            let position = Position(0, start_pos);
            let path: Option<Path> = find_path_dfs(&land, &mut visited, &position);
            if let Some(p) = path {
                return Some(p);
            }
        }
    }

    return Option::None;
}

use std::collections::HashSet;

impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let mut land = vec![vec![0; col as usize]; row as usize];
        let mut previous_path: Option<HashSet<Position>> = Option::None;
        for (cell_i, cell) in cells.iter().enumerate() {
            let cell: Vec<usize> = cell.iter().map(|it| (*it - 1) as usize).collect();
            // println!("At cell {} {}", cell[0], cell[1]);
            
            land[cell[0]][cell[1]] = 1;

            if let Some(ref p) = previous_path {
                if p.contains(&Position(cell[0], cell[1])) {
                    previous_path = Option::None;
                } else {
                    // println!("Already has path: {previous_path:?}")
                }
            }

            if previous_path == Option::None {
                let new_path = find_path(&land);

                if let Some(p) = new_path {
                    previous_path = Some(HashSet::from_iter(p.into_iter()));
                    // println!("Found path: {previous_path:?}");
                } else {
                    // println!("Failed at {cell_i}");
                    return cell_i as i32;
                }
            }
        }
        
        panic!("It should not reach here");
    }
}