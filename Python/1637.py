# 1637. Widest Vertical Area Between Two Points Containing No Points
# https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/

class Solution:
  def maxWidthOfVerticalArea(self, points: List[List[int]]) -> int:
    arr = list(map(lambda a: a[0], points))
    arr.sort()
    _max = 0

    for i in range(1, len(arr)):
      if (arr[i] == arr[i - 1]):
        continue

      _max = max(_max, arr[i] - arr[i - 1])

    return _max
