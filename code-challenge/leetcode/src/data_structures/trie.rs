
use std::collections::HashMap;

struct Node {
    value: char,
    children: HashMap<char, * mut Node>,
    fail: * const Node,
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
    root: * mut Node,
}

