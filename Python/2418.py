# 2418. Sort the People
# https://leetcode.com/problems/sort-the-people/

class Solution:
  def sortPeople(self, names: List[str], heights: List[int]) -> List[str]:
    zipped = sorted(zip(heights, names), reverse=True, key=lambda v: v[0])
    return list(list(zip(*zipped))[1])
