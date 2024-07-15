use std::io;

fn main() {
    let mut unit_old = String::new();
    let mut value = String::new();
    println!("Which units will you be converting from?: (c or f)");
    io::stdin().read_line(&mut unit_old)
        .expect("Failed to read line");
    println!("What is the value you would like to convert?:");
    io::stdin().read_line(&mut value)
        .expect("Failed to read line");
    
    let unit_old = unit_old.trim();

    match unit_old {
        "c" => {
            let value: f64 = value.trim().parse()
                .expect("Please type a number!");
            let value_new = value * 9.0 / 5.0 + 32.0;
            println!("{} degrees Celsius is {} degrees Fahrenheit", value, value_new);
        }
        "f" => {
            let value: f64 = value.trim().parse()
                .expect("Please type a number!");
            let value_new = (value - 32.0) * 5.0 / 9.0;
            println!("{} degrees Fahrenheit is {} degrees Celsius", value, value_new);
        }
        _ => {
            println!("Please type either 'c' or 'f' for the units you would like to convert from.");
        }
    }
}
