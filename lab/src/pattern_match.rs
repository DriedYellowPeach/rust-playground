
fn take_i32(x: i32) {
    println!("take_i32: {}", x);
}

fn take_i32_but_by_reference(&x: &i32) {
    let y = x;
    println!("take i32, but the parameter should be reference {}", y);
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

// pattern match can be useful
// we are forced to use object.field
fn show_point(p: &Point3D) {
    println!("x: {}, y: {}, z: {}", p.x, p.y, p.z);
}

// for simple type we can use pattern match to get the field value
fn show_point_v2(&Point3D { x, y, z }: &Point3D) {
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
}
