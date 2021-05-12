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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn is_symmetric_rec(
        root1: Option<&Rc<RefCell<TreeNode>>>,
        root2: Option<&Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root1, root2) {
            // 2개 동시 option match
            (Some(root1), Some(root2)) => {
                let (root1, root2) = (root1.borrow(), root2.borrow()); // Converts from &Option<T> to Option<&T>

                Self::is_symmetric1(root1.left.as_ref(), root2.right.as_ref())
                    && root1.val == root2.val
                    && Self::is_symmetric1(root1.right.as_ref(), root2.left.as_ref())
            }
            (None, None) => true,
            _ => false,
        }
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return Self::is_symmetric_rec(root.as_ref(), root.as_ref());
    }
}
