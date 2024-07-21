/**
 * 2373. Largest Local Values in a Matrix
 * https://leetcode.com/problems/largest-local-values-in-a-matrix/
 */

function largestLocal(grid: number[][]): number[][] {
  const N: number = grid.length;
  const out: number[][] = Array.from(Array(N - 2), () => new Array(N - 2));

  for (let row: number = 0; row < N; row++) {
    for (let col: number = 0; col < N - 2; col++) {
      grid[row][col] = Math.max(...grid[row].slice(col, col + 3));
    }
  }

  for (let col: number = 0; col < N - 2; col++) {
    for (let row: number = 0; row < N - 2; row++) {
      out[row][col] = Math.max(
        grid[row][col],
        grid[row + 1][col],
        grid[row + 2][col]
      );
    }
  }

  return out;
}
