/**
 * 2373. Largest Local Values in a Matrix
 * https://leetcode.com/problems/largest-local-values-in-a-matrix/
 */

use std::cmp::max;

impl Solution {
  pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let N = grid.len();
    let mut grid = grid;

    let mut out: Vec<Vec<i32>> = Vec::with_capacity(N - 2);

    for _ in 0..N - 2 {
      out.push(vec![0; (N-2)]);
    }

    for row in 0..N {
      for col in 0..N - 2 {
        grid[row][col] = *grid[row][col..col + 3].iter().max().unwrap();
      }
    }

    for col in 0..N - 2 {
      for row in 0..N - 2 {
        out[row][col] = max(grid[row][col], max(grid[row + 1][col], grid[row + 2][col]));
      }
    }

    return out;
  }
}
