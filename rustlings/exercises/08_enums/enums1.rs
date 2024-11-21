#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize,
    Move,
    Echo,
    ChangeColor,
    Quit,
}

fn main() {
    println!("{:?}", Message::Resize); // prints Resize
    println!("{:?}", Message::Move); // prints Move
    println!("{:?}", Message::Echo); // prints Echo
    println!("{:?}", Message::ChangeColor); // prints ChangeColor
    println!("{:?}", Message::Quit); // prints Quit
}
