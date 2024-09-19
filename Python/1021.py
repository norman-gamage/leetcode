# 1021. Remove Outermost Parentheses
# https://leetcode.com/problems/remove-outermost-parentheses/

class Solution:
  def removeOuterParentheses(self, s: str) -> str:
    cnt = 0
    out = ""

    for c in s:
      if (c == '('):
        cnt += 1
      else:
        cnt -= 1

      if ((c == '(' and cnt == 1) or (c == ')' and cnt == 0)):
        continue

      out += c

    return out
