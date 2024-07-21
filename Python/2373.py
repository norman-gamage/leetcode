# 2373. Largest Local Values in a Matrix
# https://leetcode.com/problems/largest-local-values-in-a-matrix/

class Solution:
  def largestLocal(self, grid: List[List[int]]) -> List[List[int]]:
    N = len(grid)
    out = [[0] * (N - 2) for i in range(N - 2)]

    for row in range(0, N):
      for col in range(0, N - 2):
        grid[row][col] = max(
          grid[row][col], grid[row][col + 1], grid[row][col + 2]
        )

    for col in range(0, N - 2):
      for row in range(0, N - 2):
        out[row][col] = max(
          grid[row][col], grid[row + 1][col], grid[row + 2][col]
        )

    return out
