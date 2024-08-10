# 1688. Count of Matches in Tournament
# https://leetcode.com/problems/count-of-matches-in-tournament/

class Solution:
  def numberOfMatches(self, n: int) -> int:
    out = 0

    while n > 1:
      carry = n % 2
      n //= 2
      out += n + carry

    return out
