# 1431. Kids With the Greatest Number of Candies
# https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/

class Solution:
  def kidsWithCandies(self, candies: List[int], extraCandies: int) -> List[bool]:
    out = []
    _max = max(candies)

    for v in candies:
      out.append(v + extraCandies >= _max)

    return out
