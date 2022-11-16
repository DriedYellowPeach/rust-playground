use std::ops::Deref;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_borrow_more_than_once() {
    let mut s = String::from("hello");
    let rs: &mut String = &mut s;

    // s.push_str(" world"); /* second borrow happend here */
    rs.push_str("!");
}

// owner and ref, can't modify through owner
// owner and mut ref, can't use owner at all
#[test]
fn test_mut_ref_and_owner() {
    let mut x = 5i32;
    let rx = &x; /* borrow occurs here */
    // x = 6; /* can't assign borrowed value */
    assert_eq!(*rx, x);

    let mut y = 10i32;
    let ry = &mut y;
    // assert!(y == 10); /* used of borrowed y, cannot use y because it was mutably borrowed */
    *ry = 15;

    let z = 15i32;
    let rz = &z;

    assert_eq!(z, *rz);
}

// why println! works with String and &String;
#[test]
fn test_println_with_ref() {
    let s = String::from("hello");
    let rs = &s;
    println!("{} {}", s, rs);
}

// rwr a mutable ref
#[test]
fn test_rwr_a_mut_ref() {
    let rs = &mut String::from("hello");
    assert_eq!(rs.capacity(), 5); /* unused mut */
    rs.push_str(" world");
    assert_eq!(rs, &"hello world");
    assert_eq!(rs.capacity(), 11);
}

#[test]
fn test_use_ref_of_moved() {
    let s = String::new();
    let rs = &s;
    // let ss = s; /* can't move because `s` is borrowed */
    assert_eq!(rs, "");
}

// . operator will do implicitly deref or make ref
// what happend when a method to move the value of a implicitly derefed reference
// (*rp).move_person()
// you just can't move the value behind the shared reference

// why can't move by deref the shared reference? 
// I think, ref has less priority than mut ref, and owner. It's too difficult to tract the value's liften by symbol *r
// ref can't have that priority.
// let a variable and move of that variable can only happed in single context!
#[test]
fn test_implicit_deref() {
    struct Person(i32);
    
    let p = Person(5i32);

    impl Person {
        fn move_person(self) -> Self {
            self
        }
    }

    let rp = &p;
    // rp.move_person(); /* cannot move out of `*rp` which is behind a shared reference */
    // let rp2 = *rp;
}

#[test]
fn test_possibly_borrowed() {
    let x = String::new();
    let y = String::new();
    let mut r = &x;

    if true {
        r = &y;
    }

    // you can't move here, because x maybe borrowed, and for compiler, it must be borrowed.
    // let rs = x;
    assert_eq!(r, "hello");
}

// #[test]
// fn test_iter() {
//     let x = vec![1, 2, 4];
//     let mut iterator = x.iter();
//     let y = x;

//     assert_eq!(iterator.next(), Some(&1));
// }


// this a an reborrow example
#[test]
fn test_loop() {
    let mut s = String::from("hello");
    fn take(s: &mut String) {

    }
    let ss = &mut s;
    take(ss);
    println!("{}", ss);

    for i in (0..5) {
        take(ss);
    }
}

// test smart pointer
#[test]
fn test_smart_pointer() {
    fn display(s: &str) {
        println!("{}", s);
    }

    let s = String::from("hello");
    let bs = Box::new(s);
    display(&bs);
}

#[test]
fn test_println_take_reference() {
    let s1 = "hello".to_string();
    let s2 = "world!".to_string();

    // println!("{:p}", &s1);
    // println!("{:p}", &s2);
    println!("{:p}", &s1 as &str);
    println!("{:p}", &s2 as &str);
}