# 2520. Count the Digits That Divide a Number
# https://leetcode.com/problems/count-the-digits-that-divide-a-number/

class Solution:
  def countDigits(self, num: int) -> int:
    _num = num
    out = 0

    while (_num > 0):
      lastDigit = _num % 10

      if (num % lastDigit == 0):
        out += 1

      _num //= 10

    return out
