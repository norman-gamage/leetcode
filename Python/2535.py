# 2535. Difference Between Element Sum and Digit Sum of an Array
# https://leetcode.com/problems/difference-between-element-sum-and-digit-sum-of-an-array/

class Solution:
  def differenceOfSum(self, nums: List[int]) -> int:
    element_sum = 0
    digit_sum = 0

    for n in nums:
      element_sum += n

      while (n > 0):
        digit_sum += n % 10
        n //= 10

    return element_sum - digit_sum
