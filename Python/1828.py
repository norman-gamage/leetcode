# 1828. Queries on Number of Points Inside a Circle
# https://leetcode.com/problems/queries-on-number-of-points-inside-a-circle/

class Solution:
  def countPoints(self, points: List[List[int]], queries: List[List[int]]) -> List[int]:
    out = []

    for _, q in enumerate(queries):
      cnt = 0

      for _, p in enumerate(points):
        if ((pow(p[0] - q[0], 2) + pow(p[1] - q[1], 2)) <= pow(q[2], 2)):
          cnt += 1

      out.append(cnt)

    return out
