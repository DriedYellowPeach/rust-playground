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
    //let ss = s;
    assert_eq!(rs, "");
}

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
    let rs = x;
    assert_eq!(r, "hello");
}