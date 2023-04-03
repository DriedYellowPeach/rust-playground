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
            let child_node = unsafe { &*child };
            child_node.display_with_depth(depth + 1);
        }
    }

    fn insert(&mut self, s: &str) {
        let mut cur = self;
        for c in s.chars() {
            // we leak the box here
            let next = *cur
                .children
                .entry(c)
                .or_insert_with(|| Box::into_raw(Box::new(Node::new(c))));
            cur = unsafe { &mut *next };
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
            }
            None => false,
        }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        // we recycle the box here
        for (_, &child) in self.children.iter() {
            let child_node = unsafe { Box::from_raw(child) };
            drop(child_node);
        }
    }
}

struct Trie {
    root: Node,
}

impl Trie {
    fn new() -> Self { Trie { root: Node::new('^'),
        }
    }

    fn insert(&mut self, s: &str) {
        self.root.insert(s);
    }

    // print the trie
    fn display(&self) {
        self.root.display_with_depth(0);
    }

    fn search(&self, pattern: &str) -> bool {
        true
    }
}

impl Drop for Trie {
    fn drop(&mut self) {}
}

#[cfg(test)]
mod trie_tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("hallo");
        trie.insert("hala");
        trie.display();
    }
}
