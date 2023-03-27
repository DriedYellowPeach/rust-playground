pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod v1 {
    use std::fmt::Debug;

    pub struct Node<T> {
        next: Option<Box<Node<T>>>,
        value: T,
    }

    impl<T> Node<T> {
        pub fn new(value: T) -> Self {
            Self { next: None, value }
        }
    }

    pub struct LinkedList<T> {
        head: Option<Box<Node<T>>>,
        // tail: Option<Rc<Node<T>>>,
    }

    impl<T> LinkedList<T> {
        pub fn new() -> Self {
            Self {
                head: None,
                // tail: None,
            }
        }

        pub fn push_front(&mut self, val: T) {
            match self.head.take() {
                Some(node) => {
                    let new_node = Box::new(Node {
                        next: Some(node),
                        value: val,
                    });
                    self.head = Some(new_node);
                }
                None => {
                    let new_node = Box::new(Node::new(val));
                    self.head = Some(new_node);
                }
            }
        }
    }

    impl<T> Debug for LinkedList<T>
    where
        T: Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut node = &self.head;
            write!(f, "[")?;
            while let Some(n) = node {
                write!(f, "{:?}, ", n.value)?;
                node = &n.next;
            }
            write!(f, "]")
        }
    }
}

pub mod v2 {
    use std::cell::RefCell;
    use std::fmt::Debug;
    use std::rc::Rc;

    pub struct Node<T> {
        pub next: Option<Rc<RefCell<Node<T>>>>,
        pub prev: Option<Rc<RefCell<Node<T>>>>,
        pub value: T,
    }

    impl<T> Node<T> {
        pub fn new(val: T) -> Self {
            Self {
                next: None,
                prev: None,
                value: val,
            }
        }
    }

    impl<T> Debug for Node<T>
    where
        T: Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Node: {:?}", self.value)
        }
    }

    pub struct LinkedList<T> {
        pub head: Option<Rc<RefCell<Node<T>>>>,
        pub tail: Option<Rc<RefCell<Node<T>>>>,
    }

    impl<T> LinkedList<T> {
        pub fn new() -> Self {
            Self {
                head: None,
                tail: None,
            }
        }

        pub fn push_front(&mut self, val: T) {
            let new_node = Rc::new(RefCell::new(Node::new(val)));
            match self.head.take() {
                Some(old) => {
                    self.head = Some(new_node.clone());
                    old.borrow_mut().prev = Some(new_node.clone());
                    new_node.borrow_mut().next = Some(old);
                }
                None => {
                    self.tail = Some(new_node.clone());
                    self.head = Some(new_node);
                }
            }
        }

        pub fn push_back(&mut self, val: T) {
            let new_node = Rc::new(RefCell::new(Node::new(val)));
            match self.tail.take() {
                Some(old) => {
                    old.borrow_mut().next = Some(new_node.clone());
                    new_node.borrow_mut().prev = Some(old);
                    self.tail = Some(new_node);
                }
                None => {
                    self.head = Some(new_node.clone());
                    self.tail = Some(new_node);
                }
            }
        }
    }

    impl<T: Debug> Debug for LinkedList<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "linked_list_v2: [")?;
            let mut ptr = self.head.clone();
            while let Some(n) = ptr {
                write!(f, "{:?}, ", n.borrow().value)?;
                ptr = n.borrow().next.clone();
            }
            write!(f, "]")
        }
    }
}

// I've given thee courtesy enough
mod v3 {
    use std::fmt::Debug;
    use std::ops::Drop;
    use std::ptr;

    struct Node<T> {
        prev: *mut Node<T>,
        next: *mut Node<T>,
        value: T,
    }

    impl<T> Node<T> {
        fn new(value: T) -> Self {
            Self {
                prev: ptr::null_mut(),
                next: ptr::null_mut(),
                value,
            }
        }
    }

    pub struct LinkedList<T> 
    where T: Debug
    {
        head: *mut Node<T>,
        tail: *mut Node<T>,
    }

    impl<T: Debug> LinkedList<T> {
        pub fn new() -> Self {
            Self {
                head: ptr::null_mut(),
                tail: ptr::null_mut(),
            }
        }

        pub fn push_front(&mut self, val: T) {
            let mut new_node = Box::new(Node::new(val));

            if self.head.is_null() {
                let ptr = Box::into_raw(new_node);
                self.head = ptr;
                self.tail = ptr;
            } else {
                let old_head = self.head;

                new_node.next = old_head;
                let ptr = Box::into_raw(new_node);
                unsafe {
                    // safe here because old_head and new_node must be init
                    (*old_head).prev = ptr;
                }

                self.head = ptr;
            }
        }

        pub fn push_back(&mut self, val: T) {
            let mut new_node = Box::new(Node::new(val));

            if self.tail.is_null() {
                let ptr = Box::into_raw(new_node);
                self.head = ptr; 
                self.tail = ptr;
            } else {
                let old_tail = self.tail;

                new_node.prev = old_tail;
                let ptr = Box::into_raw(new_node);
                unsafe {
                    // safe here because old_tail and new_node must be init
                    (*old_tail).next = ptr;
                }
                
                self.tail = ptr;

            }
        }
    }

    impl<T: Debug> Debug for LinkedList<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "linked_list_v2: [")?;
            // write!(f, "{:?}, ", n.borrow().value)?;
            let mut ptr = self.head;
            while !ptr.is_null() {
                // safe here because ptr is not null
                // write!(f, "{:?}, ", unsafe { &(*ptr).value })?;
                unsafe {
                    write!(f, "{:?}, ", (*ptr).value)?;
                    ptr = (*ptr).next;
                }
            }
            write!(f, "]")
        }
    }

    impl<T: Debug> Drop for LinkedList<T> {
        fn drop(&mut self) {
            let mut ptr = self.head;
            while !ptr.is_null() {
                let b = unsafe {
                    Box::from_raw(ptr)
                };
                ptr = b.next;
                println!("drop {:?}", b.value);
                drop(b)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn linked_list() {
        let mut list = v1::LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);
        list.push_front(5);
        println!("{:?}", list);
    }

    #[test]
    fn test_linked_list_v2() {
        let mut list = v2::LinkedList::<usize>::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_front(4);
        list.push_front(5);
        list.push_front(6);
        println!("{:?}", list);
        let t = list.tail.unwrap();
        println!("{:?}", (*t).borrow().value);
        let t = t.borrow().prev.as_ref().unwrap().clone();
        println!("{:?}", t.borrow().value);
        let t = t.borrow().prev.as_ref().unwrap().clone();
        println!("{:?}", t.borrow().value);
        let t = t.borrow().prev.as_ref().unwrap().clone();
        println!("{:?}", t.borrow().value);
        let t = t.borrow().prev.as_ref().unwrap().clone();
        println!("{:?}", t.borrow().value);
        let t = t.borrow().prev.as_ref().unwrap().clone();
        println!("{:?}", t.borrow().value);
    }

    #[test]
    fn test_linked_list_v3() {
        let mut list = v3::LinkedList::<usize>::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_front(4);
        list.push_front(5);
        list.push_front(6);
        println!("{:?}", list);
    }
}
