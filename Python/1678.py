# 1678. Goal Parser Interpretation
# https://leetcode.com/problems/goal-parser-interpretation/submissions/1338178796/

class Solution:
  def interpret(self, command: str) -> str:
    return command.replace("()", "o").replace("(al)", "al")
