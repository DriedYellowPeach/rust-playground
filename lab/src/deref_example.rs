
use std::fmt::Display;
use std::ops::{Deref, DerefMut};

pub struct Selector<T> {
    current: usize, 
    elements: Vec<T>,
}

// what does deref do? It means this type can be used as another type
impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.elements[self.current]
    }
}

// what does deref_mut do?
// It means this type can be used as another type
impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements[self.current]
    }
}

fn show(s: &str) -> &str {
    println!("{}", s);
    s
}

fn show2<T: Display + ?Sized>(s: &T) -> &T {
    println!("{}", s);
    s
}

#[test]
fn test_deref_corecion_in_function() {
    let mut slct = Selector {
        current: 0,
        elements: vec!["hello", "world"],
    };

    // must of type &T, and T impl Deref<Target=U>
    assert_eq!(show(&slct), "hello");
    slct.current += 1;
    assert_eq!(show(&slct), "world");
}

#[test]
fn test_deref_corecion_in_generic_funcion() {
    let mut slct = Selector {
        current: 0,
        elements: vec!["hello", "world"],
    };
    // assert_eq!(show2(&slct), "hello"); // Selectro<&str> doesn't impl std::fmt::Display;
    assert_eq!(show2(&slct as &str), "hello");
    slct.current += 1;
    assert_eq!(show2(&slct as &str), "world");
}
