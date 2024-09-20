# 1832. Check if the Sentence Is Pangram
# https://leetcode.com/problems/check-if-the-sentence-is-pangram/

class Solution:
  def checkIfPangram(self, sentence: str) -> bool:
    if (len(sentence) < 26):
      return False

    for c in "abcdefghijklmnopqrstuvwxyz":
      if (sentence.find(c) < 0):
        return False

    return True
