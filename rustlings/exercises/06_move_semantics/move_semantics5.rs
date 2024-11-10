#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: String) -> char {
    // This function converts the string into an iterator of characters,
    // gets the last character, and unwraps the result to return it.
    // Example:
    // For the input string "Rust is great!", it will return the character '!'
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: &String) {
    let data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(data.clone());

    string_uppercase(&data);
}
