# 1561. Maximum Number of Coins You Can Get
# https://leetcode.com/problems/maximum-number-of-coins-you-can-get/

class Solution:
  def maxCoins(self, piles: List[int]) -> int:
    piles.sort()
    idx = len(piles) // 3
    out = 0

    while (idx < len(piles)):
      out += piles[idx]
      idx += 2

    return out
