# 1470. Shuffle the Array
# https://leetcode.com/problems/shuffle-the-array/

class Solution:
  def shuffle(self, nums: List[int], n: int) -> List[int]:
    out = []

    for i in range(0, n):
      out.append(nums[i])
      out.append(nums[i + n])

    return out
