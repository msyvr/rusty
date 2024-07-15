#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    // Methods can either: 
    // 1. take ownership of self OR 
    // 2. borrow self immutably OR
    // 3. borrow self mutably
    // ... same as for any other parameter.

    // Here, we don't want to change the instance on which the
    // method is called (i.e., read the rectangle data but don't
    // alter the rectangle itself). To change the instance on
    // which the method is called, use `&mut self`.

    // The first method arg is always `&self`, an abbreviation
    // of `self: &Self`. This is unambiguous as Self is an alias
    // for the type for which the method is implemented.
    fn area(&self) -> u32 {
        self.w * self.h
    }

    fn contains(&self, another: &Rectangle) -> bool {
        self.w > another.w && self.h > another.h
    }

    fn square(size: u32) -> Self {
        Self {
            w: size,
            h: size,
        }
    }
}

fn main() {
    let r1 = Rectangle {
        w: 30, 
        h: 50,
    };
    let r2 = Rectangle {
        w: 3, 
        h: 5,
    };
    let r3 = Rectangle {
        w: 300, 
        h: 5,
    };        

    println!("{:?} has area {} (units: square pixels).", &r1, r1.area());
    println!("Can {:?} hold:\n{:?} -> {}\n{:?} -> {}", &r1, &r2, r1.contains(&r2), &r3, r1.contains(&r3));

    // Test the constructor associated function.
    let side = 7;
    println!("Use size {} to construct a square using Rectangle: {:?} with area {:?}", side, Rectangle::square(side), Rectangle::square(side).area());
}