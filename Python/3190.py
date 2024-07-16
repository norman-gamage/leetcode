# 3190. Find Minimum Operations to Make All Elements Divisible by Three
# https://leetcode.com/problems/find-minimum-operations-to-make-all-elements-divisible-by-three/

class Solution:
  def minimumOperations(self, nums: List[int]) -> int:
    out = 0

    for n in nums:
      out += min(n % 3, 3 - (n % 3))

    return out
