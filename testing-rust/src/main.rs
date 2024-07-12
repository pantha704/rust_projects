/*

cargo init       // for initialising a rust project ( just like npx create-vite/next )
cargo check      // to check for pre-compiling errors
cargo build      // to compile the rust program
cargo run        // to compile and run the rust program

tuto in yt : https://youtube.com/playlist?list=PLPoSdR46FgI412aItyJhj2bF66cudB6Qs&si=46PxwHWNpejJ5JRF

*/

fn main() {
    println!("Hello, world!");

    // Sleep for 3 seconds
    // std::thread::sleep(std::time::Duration::from_secs(3));

    // Defining variables
    let mut x: i32 = 3;     // by default "let" variables are immutable add "mut" keyword after them to make them mutable.
    x = 4;
    println!("Value of x {}",x);   // OP: 4 

    let mut x = "dad";  // double quotes store "strings" , single quotes store 'chars'. Might come handy in saving a few bits of space.
    x = "f";                  // 'f' is a char and "f" is string. mut doesnt allow the change of data types.
    println!("Value of x {}",x);   // OP: f  (string)

    
    // Shadowing 
    let x = 5;
    println!("Value of x {}",x);   // OP: 5

    let x = 'r';
    println!("Value of x {}",x);   // OP: r  (char)

    /* 
    Redeclaring same variable with let is called shadowing,
    which allows us to redefine "value" and "data type" of the variable.


    while, "mut" keyword allows us only to change the value not the data type. 
    */
    
}