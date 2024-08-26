/**
 * 2236. Root Equals Sum of Children
 * https://leetcode.com/problems/root-equals-sum-of-children/
 */

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
  pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let root: TreeNode = RefCell::into_inner(Rc::into_inner(root.unwrap()).unwrap());

    let root_val: i32 = root.val;
    let left_val: i32 = RefCell::into_inner(Rc::into_inner(root.left.unwrap()).unwrap()).val;
    let right_val: i32 = RefCell::into_inner(Rc::into_inner(root.right.unwrap()).unwrap()).val;

    return root_val == left_val + right_val;
  }
}
