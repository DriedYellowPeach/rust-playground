fn main() {
    println!("Hello, world!");
}

#[test]
fn test_for_loop_consume_vector() {
    let mut v = Vec::new();
    v.push(1i32);

    for i in v {
        println!("{}", i);
        // println!("{}", v.len()); /* borrow of moved value v */
    }
}

#[test]
fn test_struct_field_as_indexed_content() {
    struct person {
        name: String,
        y: i32,
    }

    let p = person{name: "hello".to_string(), y: 0};
    let s = p.name;
    // let p2 = p; /* p is partially moved */
}
