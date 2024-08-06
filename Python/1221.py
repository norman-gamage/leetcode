# 1221. Split a String in Balanced Strings
# https://leetcode.com/problems/split-a-string-in-balanced-strings/

class Solution:
  def balancedStringSplit(self, s: str) -> int:
    out = 0
    _sum = 0

    for _, c in enumerate(s):
      _sum += 1 if (c == "R") else -1

      if (_sum == 0):
        out += 1

    return out
