# 2485. Find the Pivot Integer
# https://leetcode.com/problems/find-the-pivot-integer/

class Solution:
  def pivotInteger(self, n: int) -> int:
    sum1 = 0
    sum2 = 0
    out = -1

    for i in range(1, n + 1):
      sum1 += i

    for i in range(1, n + 1):
      sum2 += i

      if (sum1 == sum2):
        out = i
        break

      sum1 -= i

    return out
