# 3146. Permutation Difference between Two Strings
# https://leetcode.com/problems/permutation-difference-between-two-strings/

class Solution:
  def findPermutationDifference(self, s: str, t: str) -> int:
    out = 0

    for i, c in enumerate(s):
      out += abs(i - t.find(c))

    return out
