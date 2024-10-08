# 2894. Divisible and Non-divisible Sums Difference
# https://leetcode.com/problems/divisible-and-non-divisible-sums-difference/

class Solution:
  def differenceOfSums(self, n: int, m: int) -> int:
    out = 0

    for i in range(1, n + 1):
      out += i if (i % m != 0) else -i

    return out
