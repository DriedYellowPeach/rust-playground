trait Shape {
    fn describe_shape(&self);
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    height: usize,
    width: usize,
}

impl Shape for Circle {
    fn describe_shape(&self) {
        println!("circle => with radius: {}", self.radius);
    }
}

impl Shape for Rectangle {
    fn describe_shape(&self) {
        println!("rectange => with height: {}, width: {}", self.height, self.width);
    }
}

impl<T: Shape> Shape for Box<T> {
    fn describe_shape(&self) {
        print!("Box => if the box just like real-world box, then the shape is a 'box', ");
        print!("inside the box, shape is: ");
        // (**self).describe_shape();
        // <T as Shape>::describe_shape(self as &T);
        <T as Shape>::describe_shape(self);
    }
}

fn main() {
    let circle = Circle{radius: 3.0};
    circle.describe_shape();
    let rec = Rectangle{ height: 1, width: 2 };
    let br = Box::new(rec);

    br.describe_shape();
    (*br).describe_shape();

    <Box<_> as Shape>::describe_shape(&br);
    <Rectangle as Shape>::describe_shape(&br);
}
