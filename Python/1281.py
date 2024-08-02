# 1281. Subtract the Product and Sum of Digits of an Integer
# https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/

class Solution:
  def subtractProductAndSum(self, n: int) -> int:
    product = 1
    sum = 0

    while n > 0:
      d = n % 10

      product *= d
      sum += d

      n //= 10

    return product - sum
