# 2114. Maximum Number of Words Found in Sentences
# https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/

class Solution:
  def mostWordsFound(self, sentences: List[str]) -> int:
    return max(list(map(lambda s: len(list(s.split(" "))), sentences)))
