# 1791. Find Center of Star Graph
# https://leetcode.com/problems/find-center-of-star-graph/

class Solution:
  def findCenter(self, edges: List[List[int]]) -> int:
    if (edges[0][0] == edges[1][0] or edges[0][0] == edges[1][1]):
      return edges[0][0]
    else:
      return edges[0][1]
