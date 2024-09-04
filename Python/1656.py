# 1656. Design an Ordered Stream
# https://leetcode.com/problems/design-an-ordered-stream/

class OrderedStream:
  def __init__(self, n: int):
    self.arr = [None for i in range(n)]
    self.idx = 0

  def insert(self, idKey: int, value: str) -> List[str]:
    self.arr[idKey - 1] = value

    out = []

    while (self.idx < len(self.arr) and self.arr[self.idx] != None):
      out.append(self.arr[self.idx])
      self.idx += 1

    return out
