# 2469. Convert the Temperature
# https://leetcode.com/problems/convert-the-temperature/

class Solution:
  def convertTemperature(self, celsius: float) -> List[float]:
    return [celsius + 273.15, 32 + celsius * 1.8]
