const SLOT: usize = 26;

fn main() {
    // unimplemented!();
    print!("hello");
    let words = vec!["cd".to_string(), "f".to_string(), "kl".to_string()];
    let mut stream_checker = StreamChecker::new(words);
    stream_checker.root.display();
    stream_checker.query('a');
}

struct StreamChecker {
    root: Node,
    buffer: Vec<char>,
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut root = Node::new(' ');
        for s in words.iter() {
            root.insert(s.chars().rev());
        }
        Self {
            root,
            buffer: Vec::new(),
        }
    }

    fn query(&mut self, letter: char) -> bool {
        self.buffer.push(letter);
        self.root.search(self.buffer.iter().copied().rev())
    }
}

#[test]
fn test_stream_checker() {
    let words = vec!["cd".to_string(), "f".to_string(), "kl".to_string()];
    let mut stream_checker = StreamChecker::new(words);
    stream_checker.root.display();
    assert!(!stream_checker.query('a'));
    assert!(!stream_checker.query('b'));
    assert!(!stream_checker.query('c'));
    assert!(stream_checker.query('d'));
    assert!(!stream_checker.query('e'));
    assert!(stream_checker.query('f'));
    assert!(!stream_checker.query('g'));
    assert!(!stream_checker.query('h'));
    assert!(!stream_checker.query('i'));
    assert!(!stream_checker.query('j'));
    assert!(!stream_checker.query('k'));
    assert!(stream_checker.query('l'));
}

#[test]
fn test_stream_checker_2() {
    let words = vec![
        "ab".to_string(),
        "ba".to_string(),
        "aaab".to_string(),
        "abab".to_string(),
        "baa".to_string(),
    ];
    let mut stream_checker = StreamChecker::new(words);
    stream_checker.root.display();
    assert!(!stream_checker.query('a'));
    assert!(!stream_checker.query('a'));
    assert!(!stream_checker.query('a'));
    assert!(!stream_checker.query('a'));
    assert!(!stream_checker.query('a'));
    assert!(stream_checker.query('b'));
}

struct Node {
    val: char,
    children: Vec<Option<Node>>,
    is_end: bool,
}

impl Node {
    fn new(val: char) -> Self {
        let mut children = Vec::new();
        children.resize_with(SLOT, || None);
        Self {
            val,
            children,
            is_end: false,
        }
    }

    fn insert<T: Iterator<Item = char>>(&mut self, mut word: T) {
        let ch = match word.next() {
            None => {
                self.is_end = true;
                return;
            }
            Some(c) => c,
        };

        let index = ch as usize - 'a' as usize;
        match self.children[index] {
            Some(ref mut kid) => {
                kid.insert(word);
            }
            None => {
                let mut node = Node::new(ch);
                node.insert(word);
                self.children[index] = Some(node);
            }
        }
    }

    fn search<T: Iterator<Item = char>>(&self, mut word: T) -> bool {
        if self.is_end {
            return true;
        }

        let ch = match word.next() {
            None => return false,
            Some(c) => c,
        };

        let index = ch as usize - 'a' as usize;
        match self.children[index] {
            Some(ref kid) => kid.search(word),
            None => false,
        }
    }

    fn display(&self) {
        self.depth_display(0);
    }

    fn depth_display(&self, depth: usize) {
        let indent = " ".repeat(depth * 2);
        let is_end = if self.is_end { "$" } else { "" };
        println!("{indent}{}{is_end}", self.val);
        for kid in &self.children {
            match kid {
                Some(kid) => kid.depth_display(depth + 1),
                None => (),
            }
        }
    }
}

#[test]
fn test_node() {
    let mut root = Node::new(' ');
    root.insert("aello".chars());
    root.insert("aerld".chars());
    root.insert("adrld".chars());
    root.insert("bdrld".chars());
    root.display();
}
