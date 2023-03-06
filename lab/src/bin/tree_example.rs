fn main() {
    println!("");
}

struct TreeNode<T> {
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

use std::cmp::Ordering;
use std::fmt::Debug;
impl<T: Ord + Debug> TreeNode<T> {
    fn binary_insert(&mut self, new_node: T) {
        let direction = match self.val.cmp(&new_node) {
            Ordering::Less => &mut self.right,
            Ordering::Greater => &mut self.left,
            Ordering::Equal => return,
        };

        match direction {
            Some(node) => {
                node.binary_insert(new_node);
            }
            None => {
                let new_node = Some(Box::new(TreeNode {
                    val: new_node,
                    left: None,
                    right: None,
                }));
                std::mem::replace(direction, new_node);
            }
        }
    }

    fn preorder(&self) {
        print!("{:?} ", self.val);
        if let Some(ref left) = self.left {
            left.preorder();
        }
        if let Some(ref right) = self.right {
            right.preorder();
        }
    }

    fn inorder(&self) {
        if let Some(ref left) = self.left {
            left.inorder();
        }
        print!("{:?} ", self.val);
        if let Some(ref right) = self.right {
            right.inorder();
        }
    }
}

#[test]
fn test_tree_node() {
    let b = TreeNode {
        val: 0,
        left: None,
        right: None,
    };
    let c = TreeNode {
        val: 3,
        left: None,
        right: None,
    };
    let mut a = TreeNode {
        val: 2,
        left: Some(Box::new(b)),
        right: Some(Box::new(c)),
    };

    a.inorder();
    a.binary_insert(-1);
    a.inorder();
    a.binary_insert(1);
    a.inorder();
}

