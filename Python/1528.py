# 1528. Shuffle String
# https://leetcode.com/problems/shuffle-string/

class Solution:
  def restoreString(self, s: str, indices: List[int]) -> str:
    out = [None] * len(indices)

    for (i, v) in enumerate(indices):
      out[v] = s[i]

    return "".join(out)
