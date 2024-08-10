# 1389. Create Target Array in the Given Order
# https://leetcode.com/problems/create-target-array-in-the-given-order/

class Solution:
  def createTargetArray(self, nums: List[int], index: List[int]) -> List[int]:
    out = []

    for (i, v) in enumerate(nums):
      out.insert(index[i], v)

    return out
