# 1863. Sum of All Subset XOR Totals
# https://leetcode.com/problems/sum-of-all-subset-xor-totals/

from functools import reduce

class Solution:
  def subsetXORSum(self, nums: List[int]) -> int:
    return reduce(lambda x, y: x | y, nums) << (len(nums) - 1)
