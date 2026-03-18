// pub makes this function visible outside the module
pub fn superblah() -> i32 {
    return 154;
}

// accepts a function and applies it to an argument
pub fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    return f(x);
}

// applies a function to an argument
pub fn doapply(x: i32) -> i32 {
    //|x|x*2 is a closure that doubles its argument x
    return apply(|x| x * 2, 5);
}

// traits are interfaces that define behavior
pub trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
    // Default implementation (optional)
    fn describe(&self) -> String {
        format!("Shape with area {:.2}", self.area())
    }
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {:.2}", self.radius);
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// Use as a parameter constraint (like generics with interface bounds)
pub fn print_area(shape: &impl Drawable) {
    println!("{}", shape.describe());
}

// Or with generic syntax (equivalent)
pub fn print_area2<T: Drawable>(shape: &T) {
    println!("{}", shape.describe());
}

// Dynamic dispatch (like interface references in C#/Java)
pub fn draw_all(shapes: &[&dyn Drawable]) {
    for s in shapes {
        s.draw();
    }
}
