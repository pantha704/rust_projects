/*

cargo init       // for initialising a rust project ( just like npx create-vite/next )
cargo check      // to check for pre-compiling errors ( errors before compilation )
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
    let x = 5;    // Original value
    println!("Value of x {}",x);   // OP: 5

    let x = 'r';    // Shadowed value
    println!("Value of x {}",x);   // OP: r  (char)

    /* 
    Redeclaring same variable with let is called shadowing,
    which allows us to redefine "value" and "data type" of the variable.


    while, "mut" keyword allows us only to change the value not the data type. 
    */

    const PI: f32 = 3.14159265359;
    // println!("Value of x {}",PI);

    // CHARS
    for char in "Hello".chars() {
        // println!("{}",char);
    }
// Compund Data Tyoes:
    // TUPLES   Collection of dynamic data types
    let tuple: (i32, f64, char) = (500, 6.4, 'X');
    
    let five_hundred = tuple.0;
    let six_point_four = tuple.1;
    let x = tuple.2;
    println!("Value of Five Hundred is : {}",five_hundred);
    println!("Value of Six Point Four is : {}",six_point_four);
    println!("Value of x is : {}",x);

    // ARRAYS   Collection of same data types
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    let first = arr[0];
    let second = arr[1];

    println!("first : {}, second : {}",first,second);

    for element in arr.iter() {
        // println!("element : {}",element);
    }

// Custom Data Types
    // STRUCTS
    let person = Person {
        name: String::from("John"),
        age: 20,
    };

    println!("The person's name is {}, his age is : {}",person.name,person.age);

    // ENUMS
    let light = TrafficLight::Red;

    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Slow down!"),
        TrafficLight::Green => println!("Go!"),
    };

    let x = sum_diff(2,4);
    print!("Sum: {}, Difference: {},\t",x.0,x.1);
    println!("As a raw Tuple : {:?}",x);      // {:?}  is used to print a tuple or {:#?} for pretty print.
}   

struct Person {
    name: String,
    age: u8,
}

enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

fn sum_diff(n1: i32, n2: i32) -> (i32, i32) {
    (n1+n2, n1-n2)
}