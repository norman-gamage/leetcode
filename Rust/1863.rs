/**
 * 1863. Sum of All Subset XOR Totals
 * https://leetcode.com/problems/sum-of-all-subset-xor-totals/
 */

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let shift = nums.len() - 1;

        return (
            nums
                .into_iter()
                .reduce(|a, b| a | b)
                .unwrap() << shift
        );
    }
}
