# 2942. Find Words Containing Character
# https://leetcode.com/problems/find-words-containing-character/

class Solution:
  def findWordsContaining(self, words: List[str], x: str) -> List[int]:
    out = []

    for (i, w) in enumerate(words):
      if (w.find(x) > -1):
        out.append(i)

    return out
