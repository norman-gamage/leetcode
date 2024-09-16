# 1614. Maximum Nesting Depth of the Parentheses
# https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/

class Solution:
  def maxDepth(self, s: str) -> int:
    cnt = 0
    out = 0

    for c in s:
      match c:
        case '(':
          cnt += 1

          if (cnt > out):
            out = cnt

        case ')':
          cnt -= 1

        # case _:
          # do nothing

    return out
