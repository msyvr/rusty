// use std::io;

fn main() {
    // The Fibonacci series: start with 0 and 1, then generate
    // the next number in the series by summing the preceding two
    // numbers. Continue this process indefinitely. The nth value:
    // Fib(n) = Fib(n-1) + Fib(n-2)
    // can be computed in at least two ways:
    // 1. For loop (over index range): Create the range of sequence 
    //    indices ending at n; sequentially compute each value in
    //    the series, starting with 0 and 1, where Fib(0) = 0 and Fib(1) = 1. 
    //    Return the nth (final) value.
    // 2. Recursion: define a function where result sums the
    //    previous two numbers in the series, then calculate
    //    those previous two numbers using the same function,
    //    unless the previous two numbers are 0 (val = 0) 
    //    and 1 (val = 1).

    let n = 7;
    let fib_indexed = fibonacci_indexed(n);
    let fib_recursive = fibonacci_recursive(n);

    println!("Fibonacci[{}] via index is {}, via recursion is {}", n, fib_indexed, fib_recursive);

}

// Indexed Fibonacci
fn fibonacci_indexed(n: u32) -> u32 {
    if n == 0 || n == 1 {
        dbg!({n});
        return n
    };

    let mut a = 0;
    let mut b = 1;
    let mut c = a + b;

    let mut i = 2;
    while i < n + 1 {
        c = a + b;
        a = b;
        b = c;
        i += 1;
    };
    return c      
}


// Recursive Fibonacci:
fn fibonacci_recursive(n: u32) -> u32 {
    match n {
        0 => n,
        1 => n,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
    }
}
