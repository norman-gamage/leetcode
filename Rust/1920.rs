/**
 * 1920. Build Array from Permutation
 * https://leetcode.com/problems/build-array-from-permutation/
 */

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut out: Vec<i32> = Vec::new();

        for x in &nums[..] {
            out.push(nums[*x as usize]);
        }

        return out;
    }
}
