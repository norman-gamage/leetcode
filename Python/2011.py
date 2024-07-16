# 2011. Final Value of Variable After Performing Operations
# https://leetcode.com/problems/final-value-of-variable-after-performing-operations/

class Solution:
  def finalValueAfterOperations(self, operations: List[str]) -> int:
    out = 0

    for o in operations:
      out += 1 if (o == "++X" or o == "X++") else -1

    return out
