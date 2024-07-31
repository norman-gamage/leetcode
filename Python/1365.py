# 1365. How Many Numbers Are Smaller Than the Current Number
# https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/

class Solution:
  def smallerNumbersThanCurrent(self, nums: List[int]) -> List[int]:
    out = []
    sorted_nums = sorted(nums)

    for v in nums:
      out.append(sorted_nums.index(v))

    return out
