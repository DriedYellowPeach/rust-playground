fn main() {
    println!("hello world");
}

// the reason why we need deref trait is that we want
// some reference like type to behave like normal reference
// the reference-like type for example, smart pointers.

// Deref trait have one method deref, to return a normal reference
// from the smart pointers, the normal reference refer the object smart
// pointers points.

#[test]
fn test_normal_reference() {
    let mut x = 5i32;
    let y = &mut x;

    assert_eq!(*y, 5);
    assert_eq!(x, 5);
}

#[test]
fn test_smart_pointers_box_as_ref() {
    let x = 5;
    let y = Box::new(x);

    let a: String = "hello".to_string();
    let b: Box<String> = Box::new(a);

    assert_eq!(x, 5);
    // assert_eq!(a, "hello"); // error borrow of moved value a.
    assert_eq!(*y, 5);
    assert_eq!(*b, "hello");
}

use std::ops::Deref;

#[test]
fn test_user_defined_smart_pointer() {
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> Self {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(x, 5);
    assert_eq!(*y, 5);
}

// now the problem is why deref designed like this:
// deref returns a plain reference and * operator deref the plain
// reference. what happend when deref returns value?

// the address of data inside box
// move of the heap value can be done with copy or move
// the difference is just the previous value can still be used
// here, i32 inside b can still be used, String inside nb can't be used.
// move of heap value all moved to stack.
#[test]
fn test_address_inside_box() {
    // value have copy trait
    let b = Box::new(5i32);
    println!("box on stack: {:p}", &b);
    println!("i32 on heap: {:p}", &*b);

    let a = *b;
    println!("a on stack: {:p}", &a);
    println!("i32 on heap: {:p}", &*b);

    // value moved
    let nb = Box::new(String::new());
    println!("box on stack: {:p}", &nb);
    println!("string on heap: {:p}", &*nb);

    let na = *nb;
    println!("a on stack: {:p}", &na);
    // println!("string on heap: {:p}", &*nb); // nb have been moved
}

// what happend if we have Deref trait return Target instead of &Target
// the box is auto destructed, we can't use box any more, and this is not good.
// but we can have an unbox function to destruct the box
fn unbox<T>(value: Box<T>) -> T {
    *value
}

#[test]
fn test_unbox() {
    let b = Box::new(String::from("hello"));
    let inner = unbox(b);
    // assert_eq!(*b, "hello"); // borrow of moved value b
    assert_eq!(inner, "hello");
}

// deref coercion is done by compiler implicitly
// and can be done many times.
// the reason why we need this is if a pointer type can 
// convert to another pointer type, and we need another type, so we do it automatically

#[test]
fn test_deref_coercion() {
    fn hello(s: &str) {
        println!("hello, {}", s);
    }

    // implict conversion &String -> &str
    let s = String::from("me");
    hello(&s);
    hello(&s[..]);
    
    // &Box<String> -> &String -> &str
    let m = Box::new(String::from("Rust"));
    hello(&m);
    hello(&(m.deref())[..]);
}

// rules of deref coercion
// &T -> &U
// &mut T -> &mut U
// &mut T -> &U
// x &T -> &mut U
use std::ops::DerefMut;

#[test]
fn test_deref_mut() {
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> Self {
            MyBox::<T>(x)
        }
    }
    
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> DerefMut for MyBox<T> {

        fn deref_mut(&mut self) -> &mut Self::Target {
           &mut self.0 
        }
    }
}
