/**
 * 2006. Count Number of Pairs With Absolute Difference K
 * https://leetcode.com/problems/count-number-of-pairs-with-absolute-difference-k/
 */

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let len: usize = nums.len();
        let mut out: i32 = 0;

        for i in 0..len-1 {
          for j in i+1..len {
            if ((nums[i]-nums[j]).abs()==k) {
              out += 1;
            }
          }
        }

        return out;
    }
}