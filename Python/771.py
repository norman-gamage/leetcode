# 771. Jewels and Stones
# https://leetcode.com/problems/jewels-and-stones/

class Solution:
  def numJewelsInStones(self, jewels: str, stones: str) -> int:
    out = 0

    for s in stones:
      if (jewels.find(s) > -1):
        out += 1

    return out
