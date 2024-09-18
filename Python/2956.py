# 2956. Find Common Elements Between Two Arrays
# https://leetcode.com/problems/find-common-elements-between-two-arrays/

class Solution:
  def findIntersectionValues(self, nums1: List[int], nums2: List[int]) -> List[int]:
    ans1 = len(list(filter(lambda n: n in nums2, nums1)))
    ans2 = len(list(filter(lambda n: n in nums1, nums2)))
    return [ans1, ans2]
