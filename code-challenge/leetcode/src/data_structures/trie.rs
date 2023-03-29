use std::collections::HashMap;

struct Node {
    value: char,
    children: HashMap<char, *mut Node>,
    fail: *const Node,
}

impl Node {
    fn new(value: char) -> Self {
        Node {
            value,
            children: HashMap::new(),
            fail: std::ptr::null(),
        }
    }
}

struct Trie {
    root: *mut Node,
}

impl Trie {
    fn new() -> Self {
        // leak the box here
        let root_ptr = Box::into_raw(Box::new(Node::new('^')));
        Trie { root: root_ptr }
    }

    fn insert(&mut self, word: String) {
        let mut cur = self.root;
        for c in word.chars() {
            let cur_node = unsafe { &mut *cur };
            // go to next child if exist
            // if not, leak the box and insert first
            let next_node = *cur_node.children.entry(c).or_insert_with(|| Box::into_raw(Box::new(Node::new(c))));
            cur = next_node;
        }
    }

    // print the trie
    fn display(&self) {

    }
}

#[cfg(test)]
mod trie_tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut trie = Trie::new();
        trie.insert("hello".to_string());
    }
}
