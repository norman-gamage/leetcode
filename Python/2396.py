# 2396. Strictly Palindromic Number
# https://leetcode.com/problems/strictly-palindromic-number/

def format_with_radix(number: int, base: int) -> list[int]:
  arr = []

  while number:
    arr.append(number % base)
    number //= base

  return list(reversed(arr))


class Solution:
  def isStrictlyPalindromic(self, n: int) -> bool:
    for i in range(2, n - 1):
      digits = format_with_radix(n, i)

      for j in range(0, 1 + math.floor(len(digits) / 2)):
        if (digits[j] != digits[len(digits) - 1 - j]):
          return False

    return True
