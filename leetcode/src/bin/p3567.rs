pub struct Solution;

fn main() {}

use std::collections::BTreeMap;

#[derive(Clone)]
struct SubMatrix {
    elements: BTreeMap<i32, usize>
}

impl SubMatrix {
    fn new() -> Self {
        Self {
            elements: BTreeMap::new()
        }
    }

    fn add(&mut self, item: i32) {
        let entry = self.elements.entry(item).or_default();
        *entry = *entry + 1;
    }

    fn remove(&mut self, item: i32) {
        let entry = self.elements.entry(item).or_default();
        if *entry == 0 {
            panic!("Removed item that does not exist");
        }
        *entry -= 1;
        if *entry == 0 {
            self.elements.remove(&item);
        }
    }

    fn min_abs_diff(&self) -> i32 {
        // println!("{:?}", self.elements.keys().collect::<Vec<_>>());
        let mut keys_iter = self.elements.keys();
        let mut prev = keys_iter.next().unwrap();
        let Some(cur) = keys_iter.next() else { return 0 };
        // print!("{} {} ", prev, cur);
        let mut min = cur - prev;
        prev = cur;
        for cur in keys_iter {
            // println!("{} {} {}", cur, prev, cur - prev);
            min = min.min(cur - prev);
            // print!("{} ", cur);
            prev = cur;
        }
        // println!();
        return min;
    }
}

impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let m = grid.len();
        let n = grid[0].len();
        let n_cols = n-k+1;
        let n_rows = m-k+1;
        let mut ans = vec![vec![0; n_cols]; n_rows];

        let mut sub_matrix = SubMatrix::new();
        let mut sub_matrices = Vec::with_capacity(n_rows);
        for k_i in 0..k {
            for k_j in 0..k {
                sub_matrix.add(grid[k_i][k_j]);
            }
        }
        ans[0][0] = sub_matrix.min_abs_diff();
        sub_matrices.push(sub_matrix.clone());
        for i in 1..n_rows {
            for k_j in 0..k {
                sub_matrix.add(grid[i+k-1][k_j]);
                sub_matrix.remove(grid[i-1][k_j]);
            }
            ans[i][0] = sub_matrix.min_abs_diff();
            sub_matrices.push(sub_matrix.clone());
        }

        for i in 0..n_rows {
            let sub_matrix = &mut sub_matrices[i];

            for j in k..n {
                for k_i in i..(i+k) {
                    sub_matrix.remove(grid[k_i][j-k]);
                    sub_matrix.add(grid[k_i][j]);
                }
                ans[i][j - k + 1] = sub_matrix.min_abs_diff();
            }
        }

        return ans;
    }
}
