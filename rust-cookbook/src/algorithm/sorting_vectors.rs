#[test]
fn test_sort_integer() {
    let mut v = [1, 4, 3, 2, 5];
    v.sort();
    println!("{:?}", v);
}

#[test]
fn test_sort_float() {
    let mut v = [1.0, 3.0, 2.0, 5.0, 4.0];
    // v.sort() won't work, because f64 impl only PartialOrder, not Ord
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", v);
}

// how to sort a struct
// 1. implement Ord trait
// 2. use sort_by
use std::cmp::{Eq, Ord};
#[derive(Debug)]
struct Person {
    uid: u64,
    name: String,
    age: u32,
    height: f64,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        if self.uid != other.uid {
            return false;
        }

        if self.name != other.name {
            return false;
        }

        if self.age != other.age {
            return false;
        }

        if self.height != other.height {
            return false;
        }

        true
    }
}

impl Eq for Person {}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.uid != other.uid {
            return self.uid.cmp(&other.uid);
        }
        if self.name != other.name {
            return self.name.cmp(&other.name);
        }
        if self.age != other.age {
            return self.age.cmp(&other.age);
        }
        if self.height != other.height {
            match self.height.partial_cmp(&other.height) {
                Some(o) => return o,
                _ => panic!("invalid data"),
            }
        }

        std::cmp::Ordering::Equal
    }
}

#[test]
fn test_sort_structs() {
    let mut v = [
        Person {
            uid: 3,
            name: "Alice".to_string(),
            age: 20,
            height: 1.7,
        },
        Person {
            uid: 2,
            name: "Bob".to_string(),
            age: 23,
            height: 1.7,
        },
        Person {
            uid: 1,
            name: "Cathy".to_string(),
            age: 19,
            height: 1.7,
        },
    ];
    v.sort();

    println!("{:?}", v);

    v.sort_by(|a, b| a.age.cmp(&b.age));
    println!("{:?}", v);
}

struct OverlappingIter<'a, T> {
    slc: &'a [T],
    current: usize,
    right: usize,
    win_sz: usize,
}

impl<'a, T> OverlappingIter<'a, T> {
    fn new(slc: &'a mut [T], win_sz: usize) -> Self {
        let len = slc.len();
        Self {
            slc,
            current: 0,
            right: len - win_sz,
            win_sz,
        }
    }
}

impl<'a, T> Iterator for OverlappingIter<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.right {
            return None;
        }

        let ret = Some(&self.slc[self.current..self.current + self.win_sz]);
        self.current += 1;
        ret
    }
}
//
#[derive(Debug)]
struct Foo<'a, 'b> {
    x: i32,
    y: &'a mut i32,
    z: &'b i32,
}

impl<'a, 'b> Foo<'a, 'b> {
    fn foo1<'c: 'a>(&'c mut self) -> &'a mut i32 {
        self.y
    }

    fn foo2(&'a mut self) -> &'a mut i32 {
        self.y
    }

    // fn foo3(&mut self) -> &'a mut i32 {
    //     &mut *self.y
    // }

    fn foo4<'c: 'a>(&'c mut self) -> &'a mut i32 {
        self.y
    }

    fn foo5<'c>(&'c mut self) -> &'a mut i32
    where
        'c: 'a,
    {
        self.y
    }

    fn foo6<'c>(&'c self) -> &'b i32
    where
        'b: 'c,
    {
        self.z
    }
}

#[test]
fn test_foo5() {
    let x = 10;
    let mut y = 11;
    let mut foo = Foo {
        x,
        y: &mut y,
        z: &x,
    };
    {
        let b = foo.foo5();
        println!("{:?}", b);
        drop(b);
    }

    drop(x);
    // drop(foo);
    drop(y);
}

#[test]
fn test_cannot_move_out_of() {
    let mut x = 10;
    let rx = &mut x;
    // drop(x);
    println!("{rx:?}");
}

#[test]
fn test_lifetime() {
    let mut y = 10; // lt: 'a
    let z = 20; // lt: 'b
                // f of type Foo<'a, 'b>
    let mut f = Foo {
        x: 1,
        y: &mut y,
        z: &z,
    };
    println!("{f:?}");
    {
        let mut y1 = 30; // lt: 'c
        f.y = &mut y1;
        // let hello = f.foo5();
        println!("{f:?}");
    }
}

fn foo7<'a, 'b, 'c>(f: &'c Foo<'a, 'b>) -> &'b i32
where
    'c: 'b,
{
    f.z
}

struct Bar<'a> {
    y: &'a i32,
}

impl<'a> Bar<'a> {
    fn bar3<'b>(&'b self) -> &'a i32
    where
        'a: 'b,
    {
        self.y
    }
}

#[test]
fn test_associated_generic_type() {
    let mut v = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let ov = OverlappingIter::new(&mut v, 3);
    // ov.map(|x| {
    //     println!("{x:?}");
    //     // x[0] += 1;
    // }).last();
}

#[test]
fn test_struct_life_time() {
    struct Fizz<'a, 'b> {
        x: &'a String,
        y: &'b String,
    }

    let world = "world".to_string();
    let r;

    {
        let hello = "hello".to_string();
        let fizz = Fizz {
            x: &world,
            y: &hello,
        };
        r = fizz.x;
    }
    println!("{:?}", r);
}
