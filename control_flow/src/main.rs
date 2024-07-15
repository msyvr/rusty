fn main() {
    let number = 7;

    // if condition must be bool; nb: `if number` alone is not an existence check in Rust
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // use condition in a let statement; nb: if/else outputs must be of the same type
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // `break` exits the current loop; `return` exits the current function
    let mut count = 0;
    let result = loop {
        count += 1;
        println!("again!");
        if count < 3 {
            continue;
        }
        println!("at last! count is at least 3");
        if count == 3 {
            break count; // loop returns count
        }
    };
    println!("The loop count is: {result}");

    count = 0;
    // labelling nested loops
        'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}