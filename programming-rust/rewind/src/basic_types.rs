// all features learns in chapter 3: basic types;

pub fn describe() {
    println!("chapter III: about rust basic types!");
}

 
#[test]
fn test_type_inference() {
    let _a = 5;
    let _b: i8 = 127;
    
    let c = (1..5).into_iter().collect::<Vec<i32>>();
}

#[test]
fn test_sequence_type() {
    let arr = [5;3];
    let arr2 = [1, 2, 3, 4];

    let arr3 = &*Box::new([1, 2, 3, 4]);
}

#[test]
fn test_vector_methods() {
    let mut v = vec![1, 2, 3, 4];
    v.insert(4, 5);
    assert_eq!(v, vec![1, 2, 3, 4, 5]);
    v.insert(1, 3);
    assert_eq!(v, vec![1, 3, 2, 3, 4, 5]);

    v.pop();
    assert_eq!(v, vec![1, 3, 2, 3, 4]);

    v.swap_remove(2);
    assert_eq!(v, vec![1, 3, 4, 3]);
}

macro_rules! myvec {
    ( $($x:expr),* ) => {
        {
            let mut v = Vec::new();
            $(v.push($x);)*
            v
        }
    }
}

#[test]
fn test_myvec() {
    let v = myvec![1, 2, 3];
    assert_eq!(v, vec![1, 2, 3]);
}

#[test]
fn test_slice() {
    let arr = [5;5];
    let s1 = &arr[..];
    
    let v: Vec<i32> = Vec::with_capacity(5);
    let s2 = &v[..];
}

#[test]
fn test_byte_string() {
    let _bs = b"hello world";
}

