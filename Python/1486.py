# 1486. XOR Operation in an Array
# https://leetcode.com/problems/xor-operation-in-an-array/

class Solution:
  def xorOperation(self, n: int, start: int) -> int:
    out = 0

    for i in range(0, n):
      out ^= start
      start += 2

    return out
