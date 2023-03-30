use std::collections::HashMap;
use std::ops::Drop;

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

    fn display_with_depth(&self, depth: usize) {
        let prefix = " ".repeat(depth);
        println!("{}{}", prefix, self.value);

        // ptr is Copy
        for (_, &child) in self.children.iter() {
            let child_node = unsafe {& *child };
            child_node.display_with_depth(depth + 1);
        }
    }

    fn search<T: Iterator<Item = char>>(&self, mut pattern: T) -> bool {
        let ch = match pattern.next() {
            Some(c) => c,
            None => return true,
        };

        match self.children.get(&ch) {
            Some(&child) => {
                let child_node = unsafe { &*child };
                child_node.search(pattern)
            },
            None => false,
        }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        // we free the node here
        for (_, &child) in self.children.iter() {
            let child_node = unsafe { Box::from_raw(child) };
            drop(child_node);
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
        if self.root.is_null() {
            println!("Empty trie");
        } else {
            let root_node = unsafe { &*self.root };
            root_node.display_with_depth(0);
        }
    }

    fn search(&self, pattern: &str) -> bool {
        true
    }
}

impl Drop for Trie {
    fn drop(&mut self) {
    }
}

#[cfg(test)]
mod trie_tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut trie = Trie::new();
        trie.insert("hello".to_string());
        trie.insert("hallo".to_string());
        trie.insert("hala".to_string());
        trie.display();
    }
}
