# 2574. Left and Right Sum Differences
# https://leetcode.com/problems/left-and-right-sum-differences/submissions/1339247246/

class Solution:
  def leftRightDifference(self, nums: List[int]) -> List[int]:
    SUM = sum(nums)
    out = [0] * len(nums)
    left_sum = 0

    for i in range(0, len(nums)):
      out[i] = abs(left_sum - (SUM - nums[i] - left_sum))
      left_sum += nums[i]

    return out
