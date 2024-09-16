# 2810. Faulty Keyboard
# https://leetcode.com/problems/faulty-keyboard/

class Solution:
  def finalString(self, s: str) -> str:
    out = ""

    for c in s:
      if c != "i":
        out += c
      else:
        out = out[::-1]

    return out
