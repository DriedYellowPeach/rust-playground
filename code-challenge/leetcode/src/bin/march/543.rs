use leetcode::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    unimplemented!();
}

fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_out = 0;
    depth(&root, &mut max_out);
    max_out
}

fn depth(node: &Option<Rc<RefCell<TreeNode>>>, max_out: &mut i32) -> i32 {
    match node {
        Some(rt) => { 
            // reborrow here
            let l = depth(&rt.borrow().left, max_out); 
            let r = depth(&rt.borrow().right, max_out);
            if *max_out < l + r {
                *max_out = l + r;
            }
            1 + std::cmp::max(l, r) 
        },
        None => 0,
    }
}

#[test]
fn test_depth() {
    let a = Option::Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: None,
        right: None,
    })));

    let b = Option::Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: None,
        right: None,
    })));

    let c = Option::Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: a,
        right: b,
    })));

    let d = Option::Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: None,
        right: None,
    })));

    let e = Option::Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: c.clone(),
        right: d,
    })));
    let mut max_out = 0;

    assert_eq!(depth(&e, &mut max_out), 3);
    assert_eq!(max_out, 3);
    max_out = 0;
    assert_eq!(depth(&c, &mut max_out), 2);
    assert_eq!(max_out, 2);
    assert_eq!(diameter_of_binary_tree(e), 3);
}
