# 2160. Minimum Sum of Four Digit Number After Splitting Digits
# https://leetcode.com/problems/minimum-sum-of-four-digit-number-after-splitting-digits/

class Solution:
  def minimumSum(self, num: int) -> int:
    arr = []

    while (num != 0):
      arr.append(num % 10)
      num //= 10

    print(arr)

    for i in range(0, 2):
      for j in range(2, 4):
        if (arr[i] > arr[j]):
          [arr[i], arr[j]] = [arr[j], arr[i]]

    return arr[0] * 10 + arr[2] + arr[1] * 10 + arr[3]
