fn main() {

    let mut s = String::from("hello, world");

    let word_index = first_word_slice_free(&s); // word will get the value 5
    println!("slice-free word_index is: {word_index}");
    s.clear(); // this empties the String, making it equal to ""
    println!("changed the string, but word_index is still: {word_index}");

    // ! The index got out of sync with the data.
    // i.e., a value is computed from data in a particular state,
    // Using string slices ensures the above 'bug' is caught at compile time;
    // the above isn't (it runs and the bug is a silent bug as the
    // code runs but the meaning of the word_index variable has changed).

    // String slices are a reference to part of a String via indices.
    // nb! string slice range indices must occur at valid UTF-8
    // character boundaries.

    let word_index = first_word_arg_string_type(&s); // word will get the value 5
    println!("slices word_index is: {word_index}");
    // s.clear(); // can't do this now as &s is borrowed as immutable
    // println!("changed the string, but word_index is still: {word_index}");

    let word_index = first_word_arg_string_type(&mut s); // word will get the value 5
    println!("slices word_index is: {word_index}");
    // s.clear(); // still can't do this now as word is tied to s, already borrowed as mutable
    // println!("changed the string, but word_index is still: {word_index}");

    let my_string = String::from("hello hello, world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    println!("partial slice of String word_index is: {word}");

    let word = first_word(&my_string[..]);
    println!("full slice of String word_index is: {word}");
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);
    println!("String reference word_index is: {word}");


    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    println!("partial slice of string literal (ref) word_index is: {word}");

    let word = first_word(&my_string_literal[..]);
    println!("full slice of string literal (ref) word_index is: {word}");


    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);        
    println!("string literal (which is a string slice by defn!) word_index is: {word}");

    // Other types of slices (not String)
    let a = [1, 2, 3, 4, 5];
    println!("a is {:?}", a);
    let slice = &a[1..3];
    println!("slice is {:?}", slice);
    let is_equal = assert_eq!(slice, &[2, 3]);
    println!("is equal is {:?}", is_equal);
    println!("Is {:?} equal to a[1..3]?: {:?}", slice, is_equal);

}

// From a string of words separated by spaces, find
// the first word (preceding the first space). If no
// spaces, the entire string is the first word.

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_arg_string_type(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Also, without slices.
// Why does the example return a usize, not a String?
fn first_word_slice_free(s: &String) -> usize {
    // Accept &String as ownership isn't needed.
    // What to return? It'll be part of a String.
    // (ed: hmmm, I was thinking of returning a new string literal?)
    // We could return the (pointer to the) index of 
    // the end of the word; ie, index of the first space.
    // Returning usize is problematic as it may not be
    // valid in the future, beyond the context of the String
    // as passed to the function.
    // (ed: a concern only if String is mutable? the function
    // signature indicates it's not mutable, right? ah, but 
    // could pass the function a reference to a mutable String)

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i; // if space found, return its position...
        }
    }
    // nb: enumerate wraps the result of iter, returning
    // each element as part of a tuple

    s.len() // ...else, return the final index of the string
}


