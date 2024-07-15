use std::io;

fn main() {

    let tup = ("hello", 2.5, 10, 'z');
    println!("The values in tup are: {}, {}, {}, {}", tup.0, tup.1, tup.2, tup.3);
    let (a, b, c, d) = tup;
    println!("The values in tup are: {}, {}, {}, {}", a, b, c, d);

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}