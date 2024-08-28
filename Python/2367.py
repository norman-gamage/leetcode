# 2367. Number of Arithmetic Triplets
# https://leetcode.com/problems/number-of-arithmetic-triplets/

class Solution:
  def arithmeticTriplets(self, nums: List[int], diff: int) -> int:
    length = len(nums)
    out = 0

    for i in range(0, length - 2):
      for j in range(i + 1, length - 1):
        if (nums[j] - nums[i] < diff):
          continue
        elif (nums[j] - nums[i] > diff):
          break
        else:
          for k in range(j + 1, length):
            if (nums[k] - nums[j] < diff):
              continue
            elif (nums[k] - nums[j] > diff):
              break
            else:
              out += 1
              break

    return out
