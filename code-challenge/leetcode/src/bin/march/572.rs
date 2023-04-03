use std::cell::RefCell;

fn main() {}

use std::rc::Rc;
#[derive(Debug, PartialEq, Eq)]
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

fn is_subtree(
    root: Option<Rc<RefCell<TreeNode>>>,
    sub_root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    check_subtree(&root, &sub_root)
}

fn check_subtree(
    root: &Option<Rc<RefCell<TreeNode>>>,
    sub_root: &Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    // compare with root
    if is_same_tree(root, sub_root) {
        return true;
    }

    if let Some(tr) = root {
        // then root->left
        if check_subtree(&tr.borrow().left, sub_root) {
            return true;
        }

        // then root->right
        if check_subtree(&tr.borrow().right, sub_root) {
            return true;
        }
    }

    false
}

fn is_same_tree(a: &Option<Rc<RefCell<TreeNode>>>, b: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(ta), Some(tb)) => {
            // println!("{}, {}", ta.borrow().val, tb.borrow().val);
            if ta.borrow().val != tb.borrow().val {
                false
            } else {
                is_same_tree(&ta.borrow().right, &tb.borrow().right)
                    && is_same_tree(&ta.borrow().left, &tb.borrow().left)
            }
        }
        _ => false,
    }
}

#[test]
fn test_is_subtree() {
    let a = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    })));

    let b = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: None,
        right: None,
    })));

    let c = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: a,
        right: b,
    })));

    let d = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: None,
        right: None,
    })));

    let e = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: c,
        right: d,
    })));

    let a1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    })));

    let b1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: None,
        right: None,
    })));

    let c1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: a1,
        right: b1,
    })));

    assert!(is_subtree(e, c1));
}
