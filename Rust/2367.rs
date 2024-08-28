/**
 * 2367. Number of Arithmetic Triplets
 * https://leetcode.com/problems/number-of-arithmetic-triplets/
 */

impl Solution {
  pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
    let len: usize = nums.len();
    let mut out: i32 = 0;

    for i in 0..len - 2 {
      for j in i + 1..len - 1 {
        if nums[j] - nums[i] < diff {
          continue;
        } else if nums[j] - nums[i] > diff {
          break;
        } else {
          for k in j + 1..len {
            // println!("{:?} {:?} {:?}", nums[i], nums[j], nums[k]);
            if nums[k] - nums[j] < diff {
              continue;
            } else if nums[k] - nums[j] > diff {
              break;
            } else {
              out += 1;
              break;
            }
          }
        }
      }
    }

    return out;
  }
}
