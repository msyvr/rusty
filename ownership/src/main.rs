fn main() {
    // Variable scope: valid in the scope where it's introduced
    // eg: 's' will be valid for the entirety of `main`.
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    // Multiple copies of a value: int vs String
    // TL;DR: 
    // - Values on the stack are *copied*.
    // - Values on the heap are *moved* unless `.clone` is invoked.
    let x = 5;
    let y = x;
    
    {
        let mut z = x;
        z = 2 * z;
        println!("{z}")

    }
    println!("{x}");
    println!("{y}");
    // println!("{z}"); // Error: out of scope; no longer exists.

    // What happens for String type (vs int)?
    let s1 = String::from("hello");
    let s2 = s1; 
    // For a value stored on the heap, this assignment copies the 
    // stack values of s1: ptr, len, capacity. It does not copy
    // the associated value on the heap (at the pointer address).
    // Q: When a variable goes out of scope, `drop` is called and 
    // heap memory cleaned up. What happens if s1 and s2 are in 
    // different scopes? (cf "double free error")
    // Thus, s2 = s1 *moves* s1 to s2 for values on the heap.
    // When s2 goes out of scope, the heap memory is freed and
    // the value can no longer be accessed.

    println!("{s2}, world!");
    // println!("{s1}, world!"); // Error: 'value borrowed...' = does not exist.

    let s1 = String::from("hello");
    let s2 = s1.clone(); // This is a 'deep' copy: fully replicated.
    println!("{s2}, world!");
    println!("{s1}, world!");

    let s = String::from("goodbye");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5 * 100;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
    let s = String::from("goodbye again");  // s comes into scope
    let (s_returned, _s_cap) = takes_gives_ownership(s);
    println!("string var still with us?: {s_returned}");

    // Use references to pass values from the heap which will
    // be required after the function scope.

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    // Mutable references: must be the only reference to the value.
    // This enables Rust to prevent data races at compile time. Powerful!

    // Will error whether the 'other' assignments are mut or not:
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r_imm = &s;
    // let r2 = &mut s;
    // println!("{}, {}, {}", r1, r_imm, r2);

    // Yes, can use scope to allow multiple mutable references but they
    // can't be simultaneous references (same scope).

    let mut s = String::from("hello");
    println!("string in original scope as s: {s}");

    {
        let r1 = &mut s;
        r1.push_str(" from a new scope!");
        println!("string in new scope as r1: {r1}");
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    println!("string after r1 scope ended, as r2: {r2}");
    // nb: the above assigns r2 to the mutated s (ie: "hello from a new scope!")

    // Multiple immutable references are fine: 
    // multiple independent data reads don't pose an issue.
    // Note the order below. It's fine because a reference's scope
    // ends after its last use.
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}"); 
    
    // Dangling references/pointers: Rust handles these by disallowing the possibility.   
    // let _reference_to_nothing = dangle(); // Must specify lifetime for this to work.
    let _reference_to_nothing = no_dangle();

}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn takes_gives_ownership(some_string: String) -> (String, usize) { // some_string comes into scope
    println!("{some_string}");
    let string_length = some_string.len();
    println!("String length: {string_length}");    
    let string_capacity = some_string.capacity();
    println!("String capacity: {string_capacity}");
    (some_string, string_capacity)
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn calculate_length(s: &String) -> usize {
    s.len()
}  

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}