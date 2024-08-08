# 2000. Reverse Prefix of Word
# https://leetcode.com/problems/reverse-prefix-of-word/

class Solution:
  def reversePrefix(self, word: str, ch: str) -> str:
    first_ind = word.find(ch) + 1
    return word[0:first_ind][::-1] + word[first_ind:]
