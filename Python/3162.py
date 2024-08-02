# 3162. Find the Number of Good Pairs I
# https://leetcode.com/problems/find-the-number-of-good-pairs-i/

class Solution:
  def numberOfPairs(self, nums1: List[int], nums2: List[int], k: int) -> int:
    out = 0
    nums2 = list(map(lambda n: k * n, nums2))

    for i in nums2:
      if (i == 1):
        out += len(nums1)
        continue

      for j in nums1:
        if (i > j):
          continue

        if (j % i == 0):
          out += 1

    return out
