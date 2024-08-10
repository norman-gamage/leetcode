# 1313. Decompress Run-Length Encoded List
# https://leetcode.com/problems/decompress-run-length-encoded-list/

class Solution:
  def decompressRLElist(self, nums: List[int]) -> List[int]:
    out = []

    for i in range(0, len(nums), 2):
      for j in range(0, nums[i]):
        out.append(nums[i + 1])

    return out
