use std::f64::consts::PI;


trait Shape {
    fn description(&self) -> String;
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

trait ReShape {
    fn bigger(&mut self, size: f64) -> f64;
}

struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Circle { radius }
    }
}

impl ReShape for Circle {
    fn bigger(&mut self, size: f64) -> f64 {
        self.radius += size;
        self.radius
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        PI * 2.0 * self.radius
    }

    fn description(&self) -> String {
        "shape is circle".to_string()
    }
}

fn main() {
    let mut circle = Circle::new(3.0);
    let shape : &dyn Shape = &mut circle;
    println!("shape => {}", shape.area());
    let reshape: &mut dyn ReShape = &mut circle;
    println!("reshape => {}", reshape.bigger(1.0));
    println!("end");
}
