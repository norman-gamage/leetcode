# 3065. Minimum Operations to Exceed Threshold Value I
# https://leetcode.com/problems/minimum-operations-to-exceed-threshold-value-i/

class Solution:
  def minOperations(self, nums: List[int], k: int) -> int:
    out = 0

    for v in nums:
      if (v < k):
        out += 1

    return out
