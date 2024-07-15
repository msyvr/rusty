fn main() {
    const ONE_DAY_IN_SECONDS: u32 = 1 * 24 * 60 * 60;
    println!("There are {ONE_DAY_IN_SECONDS} seconds in a day!");

    let x = 5;
    let x = x + 1;
    {
        let x = x*2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
