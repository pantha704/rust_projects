#![allow(dead_code)]

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

// The `impl` block is used to define methods and associated functions for a type, such as a struct or enum.
// It is used when you want to add functionality to a type, like methods that operate on its data.
// In this case, the `impl` block is used to define a method `call` for the `Message` enum, which prints the `Message` instance.
// that means any instance of Message enum is wrapped by this impl? answer yes or no in the new line
// yes
impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
