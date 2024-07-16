# 1920. Build Array from Permutation
# https://leetcode.com/problems/build-array-from-permutation/

class Solution:
  def buildArray(self, nums: List[int]) -> List[int]:
    out = []

    for x in nums:
      out.append(nums[x])

    return out
