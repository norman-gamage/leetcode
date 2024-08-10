# 2652. Sum Multiples
# https://leetcode.com/problems/sum-multiples/

class Solution:
  def sumOfMultiples(self, n: int) -> int:
    out = 0

    for i in range(0, n + 1):
      if (i % 3 == 0 or i % 5 == 0 or i % 7 == 0):
        out += i

    return out
