# 1512. Number of Good Pairs
# https://leetcode.com/problems/number-of-good-pairs/

class Solution:
  def numIdenticalPairs(self, nums: List[int]) -> int:
    numCount = {}
    out = 0

    for n in nums:
      if (numCount.get(n) == None):
        numCount.update({n: 1})
      else:
        numCount.update({n: 1 + numCount.get(n)})

    for v in numCount.values():
      out += int((v * (v - 1)) / 2) if {v > 1} else {0}

    return out
