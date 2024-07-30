# 2824. Count Pairs Whose Sum is Less than Target
# https://leetcode.com/problems/count-pairs-whose-sum-is-less-than-target/

class Solution:
  def countPairs(self, nums: List[int], target: int) -> int:
    out = 0

    for i in range(0, len(nums) - 1):
      for j in range(i + 1, len(nums)):
        if (nums[i] + nums[j] < target):
          out += 1

    return out
