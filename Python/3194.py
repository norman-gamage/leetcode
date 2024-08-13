# 3194. Minimum Average of Smallest and Largest Elements
# https://leetcode.com/problems/minimum-average-of-smallest-and-largest-elements/

class Solution:
  def minimumAverage(self, nums: List[int]) -> float:
    nums.sort()
    out = sys.float_info.max

    for i in range(0, len(nums) // 2):
      avg = (nums[i] + nums[len(nums) - i - 1]) / 2

      if avg < out:
        out = avg

    return out
