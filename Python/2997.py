# 2997. Minimum Number of Operations to Make Array XOR Equal to K
# https://leetcode.com/problems/minimum-number-of-operations-to-make-array-xor-equal-to-k/

class Solution:
  def minOperations(self, nums: List[int], k: int) -> int:
    return (k ^ reduce(lambda a, b: a ^ b, nums)).bit_count()
