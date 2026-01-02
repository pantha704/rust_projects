use std::process;
use std::io;
use rhai::Engine;

fn main() {
    println!("\nEnter your expression (or 'exit' to quit):");

    loop {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        
        if input.eq_ignore_ascii_case("exit") {
            process::exit(1);
        }
        
        let engine = Engine::new();
        let result = engine.eval::<i64>(input);

        match result {
            Ok(value) => {
                println!("Result: {}", value);
                main();
            },
            Err(_) => {
                print!("\x1B[2J\x1B[1;1H"); // Clear the terminal
                println!("\nError evaluating expression, please enter a valid arithmetic expression\n");
                main();
            }
        }
    }
}