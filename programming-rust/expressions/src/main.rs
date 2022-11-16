fn main() {
    println!("Hello, world!");
}

#[test]
fn test_semicolon_in_match() {
    let code = 5i32;

    let y = match code {
        0 => println!("hello"),
        _ => println!("world")
    };

    match code {
        0 => 0, 
        _ => 1
    };
}

// there is deref and deref_mut method on Box for Deref trait
// mut box can deref_mut and deref but immutable box can only deref
#[test]
fn test_method_call() {
    let mut s = String::new(); 
    let mut s_in_box = Box::new(String::from("hello"));
    s_in_box.push_str("world");

    let s_in_box_two = Box::new(String::from("hello"));
    //s_in_box_two.push('h'); /* cannot borrow `*s_in_box_two` as mutable, as `s_in_box_two` is not declared as mutable */

    let mut ss = String::from("hello");

    // automatically borrow
    ss.capacity();
    
    // automatically deref, this only make sense for Copy
    struct MyString (String);
    impl MyString {
        fn take_string(self) -> Self {
            self
        }
    }
    let ms = MyString(String::from("hello"));
    let rms = & ms;
    // rms.take_string(); /* cannot move out of `*rms` which is behind a shared reference */

    // automatically reborrow
    let mss = & mut ss;
    mss.push_str("world");

    // automatically deref coercion and borrow     
    s_in_box.push('!');
}
