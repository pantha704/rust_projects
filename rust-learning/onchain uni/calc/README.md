# Calc

Calc is a simple command-line calculator built in Rust. It allows you to evaluate arithmetic expressions interactively using the Rhai scripting engine.

## Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install)

## Installation

1. **Clone the Repository:**

   ```bash
   git clone <repository-url>
   cd calc
   ```

   Replace `<repository-url>` with your actual repository URL.

2. **Build the Project:**
   ```bash
   cargo build --release
   ```
   The release executable will be located in the `target/release` directory.

## Usage

You can run the application using Cargo or by running the built executable directly.

- **Using Cargo:**

  ```bash
  cargo run --release
  ```

- **Using the Executable:**
  ```bash
  ./target/release/calc
  ```

When you run the program, you'll see the following prompt:

```
Enter your expression (or 'exit' to quit):
```

- **Evaluate Expressions:**  
  Type any valid arithmetic expression (e.g., `1 + 2 * 3`) and press Enter. The program will evaluate the expression using the Rhai engine and display the result.

- **Exit the Program:**  
  Type `exit` and press Enter to quit the application.

## How It Works

- **Input Handling:**  
  The program reads your input from the terminal. If the input is `"exit"`, it terminates the program.

- **Expression Evaluation:**  
  The Rhai engine evaluates the provided arithmetic expression. On successful evaluation, the result is printed. On failure, the screen is cleared and an error message is shown.

- **Program Flow:**  
  After displaying the result or an error, the program re-invokes the main process to continue receiving input.

## Project Structure

```
calc/
├── Cargo.toml         # Project configuration and dependencies
├── Cargo.lock         # Auto-generated lock file (do not edit)
├── README.md          # This file
└── src/
    └── main.rs        # Main source code for the calculator
```

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests for enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
