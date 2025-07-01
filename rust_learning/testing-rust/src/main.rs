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
    println!("Value of PI {}",PI);

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
   struct Person {
      name: String,
      age: u8,
    }

    let person = Person {
        name: String::from("John"),
        age: 20,
    };

    println!("The person's name is {}, his age is : {}",person.name,person.age);

    // ENUMS 
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    let light = TrafficLight::Red;

    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Slow down!"),
        TrafficLight::Green => println!("Go!"),
    };

    let x = sum_diff(2,4);
    fn sum_diff(n1: i32, n2: i32) -> (i32, i32) {
      (n1+n2, n1-n2)
    }
    print!("Sum: {}, Difference: {},\t",x.0,x.1);
    println!("As a raw Tuple : {:?}",x);      // {:?}  is used to print a tuple/array or {:#?} for pretty print.


    // Control Flow in Rust

    let num = if 2 > 1 {
        true
    } else {
        false
    };

    println!("Output : {}",num);   // OP : true


    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_of_cent(coin : Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    let coin = Coin::Penny;

    println!("Value of coin : {}",value_of_cent(coin));

    // 'loop' runs an infinite loop until the 'break' keyword is called from inside it.
    loop { 
        println!("Yo!  ");
        break;
    }

    let arr = [1,2,3,4];
    for element in arr.iter() {
        println!("Element : {}",element);
    }

    for num in (1..4).rev() {    // 1 to 3 ( 4 excluded ) and .rev() to reverse it into 3 to 1.
        println!("{}!", num);  // OP : 3! 2! 1!
    }

    for num in 1..=100 {
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", num);
        }
    }

    // Ownerships in Rust
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is moved to s2, s1 is no longer valid
    // println!("{}", s1);  // This would cause a compile-time error

    // Borrowing
    let s3 = String::from("world");
    let len = calculate_length(&s3);  // s3 is borrowed, not moved
    println!("The length of '{}' is {}.", s3, len);

    // Slices
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    // Vectors
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("Vector: {:?}", v);

    // Hashmaps
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Scores: {:?}", scores);

    // Error Handling
    let result = divide(10, 2);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    // Closures
    let add_one = |x| x + 1;
    println!("5 + 1 = {}", add_one(5));

    // Iterators
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("Sum of numbers: {}", sum);

    // Option and Result
    let some_number = Some(5);
    let absent_number: Option<i32> = None;
    println!("Some number: {:?}", some_number);
    println!("Absent number: {:?}", absent_number);

    let result: Result<i32, &str> = Ok(10);
    match result {
        Ok(value) => println!("Result is Ok with value: {}", value),
        Err(e) => println!("Result is Err with error: {}", e),
    }

    // Pattern Matching
    let favorite_color: Option<&str> = Some("blue");
    if let Some(color) = favorite_color {
        println!("Your favorite color is {}", color);
    } else {
        println!("You don't have a favorite color");
    }

    // Traits
    trait Summary {
        fn summarize(&self) -> String;
    }

    struct Article {
        headline: String,
        content: String,
    }

    impl Summary for Article {
        fn summarize(&self) -> String {
            format!("{}: {}", self.headline, self.content)
        }
    }

    let article = Article {
        headline: String::from("Rust is awesome!"),
        content: String::from("Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety."),
    };

    println!("Article summary: {}", article.summarize());

    // Generics
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // Lifetimes
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

    // Concurrency
    use std::thread;
    use std::time::Duration;

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // Asynchronous Programming
    use tokio::time;

    async fn say_hello() {
        time::sleep(Duration::from_secs(1)).await;
        println!("hello, world!");
    }

    let future = say_hello();
    tokio::runtime::Runtime::new().unwrap().block_on(future);

    // Error Handling with `?` Operator
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading file: {}", e),
    }

    // Using `Box` for Heap Allocation
    box_example();

    // Using `Rc` and `RefCell` for Shared Ownership and Interior Mutability
    rc_refcell_example();

    // Implementing Custom Iterators
    custom_iterator_example();

    // Using `async` and `await` for Asynchronous Programming
    async_example().await;

    // Using `Arc` for Thread-Safe Shared Ownership
    arc_example();

    // Concurrency with `async-std`
    async_std_example().await;

    // Macros
    macro_example();

    // Unsafe Rust
    unsafe_example();

    // FFI (Foreign Function Interface)
    ffi_example();

    // Procedural Macros
    custom_derive_macro_example();

    // Pinning
    pinning_example();

    // Async Streams
    async_stream_example().await;

    // WebAssembly (Wasm)
    #[cfg(target_arch = "wasm32")]
    wasm_example();
}   

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

// Error Handling with `?` Operator
fn read_username_from_file() -> Result<String, std::io::Error> {
    use std::fs::File;
    use std::io::{self, Read};

    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

// Using `Box` for Heap Allocation
fn box_example() {
    let b = Box::new(5);
    println!("b = {}", b);
}

// Using `Rc` and `RefCell` for Shared Ownership and Interior Mutability
use std::rc::Rc;
use std::cell::RefCell;

fn rc_refcell_example() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::clone(&value);
    let b = Rc::clone(&value);

    *a.borrow_mut() += 10;
    println!("Value after mutation: {}", b.borrow());
}

// Implementing Custom Iterators
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn custom_iterator_example() {
    let counter = Counter::new();
    for count in counter {
        println!("Count: {}", count);
    }
}

// Using `async` and `await` for Asynchronous Programming
use tokio::time::{sleep, Duration};

async fn async_example() {
    sleep(Duration::from_secs(1)).await;
    println!("Async function executed!");
}

// Using `Arc` for Thread-Safe Shared Ownership
use std::sync::Arc;
use std::thread;

fn arc_example() {
    let value = Arc::new(5);
    let value1 = Arc::clone(&value);
    let value2 = Arc::clone(&value);

    let handle1 = thread::spawn(move || {
        println!("Value in thread 1: {}", value1);
    });

    let handle2 = thread::spawn(move || {
        println!("Value in thread 2: {}", value2);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

// Concurrency with `async-std`
use async_std::task;

async fn async_std_example() {
    task::sleep(Duration::from_secs(1)).await;
    println!("Async-std function executed!");
}

// Macros
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

fn macro_example() {
    say_hello!();
}

// Unsafe Rust
fn unsafe_example() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

// FFI (Foreign Function Interface)
extern "C" {
    fn abs(input: i32) -> i32;
}

fn ffi_example() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// Procedural Macros
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_my_macro(&ast)
}

fn impl_my_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl #name {
            pub fn hello() {
                println!("Hello, {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

// Custom Derive Macros
#[derive(MyMacro)]
struct MyStruct;

fn custom_derive_macro_example() {
    MyStruct::hello();
}

// Pinning
use std::pin::Pin;

fn pinning_example() {
    let mut data = String::from("Hello");
    let pinned_data = Pin::new(&mut data);
    println!("Pinned data: {}", pinned_data);
}

// Async Streams
use futures::stream::{self, StreamExt};

async fn async_stream_example() {
    let stream = stream::iter(vec![1, 2, 3]);
    let collected: Vec<i32> = stream.collect().await;
    println!("Collected stream: {:?}", collected);
}

// WebAssembly (Wasm)
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm!");
}

#[cfg(target_arch = "wasm32")]
fn wasm_example() {
    greet();
}

#[tokio::main]
async fn main() {
    // ... existing code ...

    // Procedural Macros
    custom_derive_macro_example();

    // Pinning
    pinning_example();

    // Async Streams
    async_stream_example().await;

    // WebAssembly (Wasm)
    #[cfg(target_arch = "wasm32")]
    wasm_example();

    // ... existing code ...
}
