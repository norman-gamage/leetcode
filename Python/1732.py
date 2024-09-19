# 1732. Find the Highest Altitude
# https://leetcode.com/problems/find-the-highest-altitude/

class Solution:
  def largestAltitude(self, gain: List[int]) -> int:
    height = 0
    out = 0

    for g in gain:
      height += g

      if (height > out):
        out = height

    return out
