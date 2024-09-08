# 2006. Count Number of Pairs With Absolute Difference K
# https://leetcode.com/problems/count-number-of-pairs-with-absolute-difference-k/

class Solution:
  def countKDifference(self, nums: List[int], k: int) -> int:
    length = len(nums)
    out = 0

    for i in range(0, length - 1):
      for j in range(i + 1, length):
        if (abs(nums[i] - nums[j]) == k):
          out += 1

    return out
