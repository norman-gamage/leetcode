# 2194. Cells in a Range on an Excel Sheet
# https://leetcode.com/problems/cells-in-a-range-on-an-excel-sheet/

class Solution:
  def cellsInRange(self, s: str) -> List[str]:
    out = []

    row_beg = ord(s[0])
    col_beg = int(s[1])
    row_end = ord(s[3])
    col_end = int(s[4])

    for i in range(row_beg, row_end + 1):
      for j in range(col_beg, col_end + 1):
        out.append(chr(i) + str(j))

    return out
