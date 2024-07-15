fn main() {
    println!("Hello, world!");
    another_function(3);

    // statement vs expression
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y (statement assigned to expression) is: {}", y);
    another_function_with_return_value(5);
    let f = five();
    println!("Oh, look! The 'five' function returns {}", f)
}

fn another_function(x: i32) {
    println!("I'm another function with argument {}.", x);
}

fn another_function_with_return_value(x: i32) -> i32 {
    println!("I'm a function with return value type specified and argument {}.", x);
    x + 1 // note: 1. no semicolon; 2. this expression returns a value
}

fn five() -> i32 {
    5
}