use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// Implementing Solution structure
pub struct Solution;

impl Solution {
    /// Returns whether the two binary trees `p` and `q` are the same.
    ///
    /// Two binary trees are considered the same if they are structurally identical, 
    /// and the nodes have the same value.
    ///
    /// # Arguments
    /// * `p` - A root of the first binary tree wrapped in `Option<Rc<RefCell<TreeNode>>>`.
    /// * `q` - A root of the second binary tree wrapped in `Option<Rc<RefCell<TreeNode>>>`.
    ///
    /// # Returns
    /// * `bool` - `true` if `p` and `q` are the same, `false` otherwise.
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                let (p, q) = (p.borrow(), q.borrow());
                p.val == q.val && Solution::is_same_tree(p.left.clone(), q.left.clone()) && Solution::is_same_tree(p.right.clone(), q.right.clone())
            },
            (None, None) => true,
            _ => false,
        }
    }
}

// Test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_tree() {
        let p = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(Solution::is_same_tree(p, q), true);

        let p = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        assert_eq!(Solution::is_same_tree(p, q), false);
    }
}

fn main() {
    let p = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let q = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    println!("Are trees same: {}", Solution::is_same_tree(p, q));
}
