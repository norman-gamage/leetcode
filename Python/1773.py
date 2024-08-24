# 1773. Count Items Matching a Rule
# https://leetcode.com/problems/count-items-matching-a-rule/

class Solution:
  def countMatches(self, items: List[List[str]], ruleKey: str, ruleValue: str) -> int:
    idx = 0
    out = 0

    if (ruleKey == "color"):
      idx = 1
    elif (ruleKey == "name"):
      idx = 2

    for v in items:
      if (ruleValue == v[idx]):
        out += 1

    return out
