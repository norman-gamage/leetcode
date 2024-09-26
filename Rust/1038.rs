/**
 * 1038. Binary Search Tree to Greater Sum Tree
 * https://leetcode.com/problems/binary-search-tree-to-greater-sum-tree/
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
  pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut root = root;

    pub fn reverse_inorder_traversal(
      node: &mut Option<Rc<RefCell<TreeNode>>>,
      mut sum: i32
    ) -> i32 {
      let mut node = node.as_ref().unwrap().borrow_mut();

      if node.right.is_some() {
        sum = reverse_inorder_traversal(&mut node.right, sum);
      }

      sum += node.val;
      node.val = sum;

      if node.left.is_some() {
        sum = reverse_inorder_traversal(&mut node.left, sum);
      }

      return sum;
    }

    reverse_inorder_traversal(&mut root, 0);

    return root;
  }
}
