use core::error;
use std::any::{self, Any};

struct Point {
    x: u64,
    y: u64,
}

enum Message {
    // TODO: Implement the message variant types based on their usage below.
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

struct State {
    width: u64,
    height: u64,
    position: Point,
    message: String,
    // RGB color composed of red, green and blue.
    color: (u8, u8, u8),
    quit: bool,
}

impl State {
    fn resize(&mut self, width: u64, height: u64) {
        self.width = width;
        self.height = height;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue);
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    // '&dyn Any' allows the function to return a reference to any type, enabling dynamic type handling.
    // Example: let value: &dyn Any = &42;

    // '&mut self' means the method can modify the instance of the struct it is called on.
    fn process(&mut self, message: Message) {
        // TODO: Create a match expression to process the different message
        // variants using the methods defined above.

        // The match expression works similarly to a switch statement in other languages.
        // It allows you to compare a value against a series of patterns and execute code based on which pattern matches.
        // Example: 
        // match value {
        //     Pattern1 => { /* code to execute if value matches Pattern1 */ },
        //     Pattern2 => { /* code to execute if value matches Pattern2 */ },
        //     _ => { /* code to execute if value doesn't match any pattern */ },
        // }
        // In this case, we match the message to the corresponding method.
        match message {
            Message::Resize { width, height } => self.resize(width, height),
            Message::Move(point) => self.move_position(point),
            Message::Echo(s) => self.echo(s),
            Message::ChangeColor(r, g, b) => self.change_color(r, g, b),
            Message::Quit => self.quit(),
            // '_' is a catch-all pattern that matches any value that doesn't match the previous patterns.
            _ => panic!("No matching message found"),
        }

    }
}

fn main() {
    // You can optionally experiment here.
    let mut state = State {
        width: 0,
        height: 0,
        position: Point { x: 0, y: 0 },
        message: String::from("hello world"),
        color: (0, 0, 0),
        quit: false,
    };
    state.process(Message::Resize { width: 10, height: 30 });
    state.process(Message::Move(Point { x: 10, y: 15 }));
    state.process(Message::Echo(String::from("Hello world!")));
    state.process(Message::ChangeColor(255, 0, 255));
    state.process(Message::Quit);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            width: 0,
            height: 0,
            position: Point { x: 0, y: 0 },
            message: String::from("hello world"),
            color: (0, 0, 0),
            quit: false,
        };

        state.process(Message::Resize {
            width: 10,
            height: 30,
        });
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Quit);

        assert_eq!(state.width, 10);
        assert_eq!(state.height, 30);
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.message, "Hello world!");
        assert_eq!(state.color, (255, 0, 255));
        assert!(state.quit);
    }
}
