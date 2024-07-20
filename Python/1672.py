# 1672. Richest Customer Wealth
# https://leetcode.com/problems/richest-customer-wealth/

class Solution:
  def maximumWealth(self, accounts: List[List[int]]) -> int:
    return max(list(map(lambda acc: (reduce(lambda a, b: a + b, acc, 0)), accounts)))
