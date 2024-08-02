# 2859. Sum of Values at Indices With K Set Bits
# https://leetcode.com/problems/sum-of-values-at-indices-with-k-set-bits/

class Solution:
  def sumIndicesWithKSetBits(self, nums: List[int], k: int) -> int:
    out = 0

    for (i, v) in enumerate(nums):
      if (i.bit_count() == k):
        out += v

    return out
