# 2108. Find First Palindromic String in the Array
# https://leetcode.com/problems/find-first-palindromic-string-in-the-array/

class Solution:
  def firstPalindrome(self, words: List[str]) -> str:
    out = ""

    for w in words:
      if (w == w[::-1]):
        out = w
        break

    return out
