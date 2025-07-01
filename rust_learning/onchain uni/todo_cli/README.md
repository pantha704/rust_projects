# ToDo CLI

ToDo CLI is a simple command-line tool for managing tasks efficiently.

## Installation

1. **Install Rust and Cargo:**

   - Follow the instructions on the [Rust website](https://www.rust-lang.org/tools/install) to install Rust and Cargo.

2. **Clone the Repository:**

   ```bash
   git clone <repository-url>
   cd todo_cli
   ```

3. **Build the Project:**

   ```bash
   cargo build --release
   ```

   The executable will be located in the `target/release` directory.

## Usage

Navigate to the directory containing the built executable and use the following commands:

- **Add a Task:**

  ```bash
  ./todo_cli -a "Buy groceries"
  ```

- **List Tasks:**

  ```bash
  ./todo_cli -l
  ```

- **Check a Task:**

  ```bash
  ./todo_cli -c 1
  ```

- **Uncheck a Task:**

  ```bash
  ./todo_cli -u 1
  ```

- **Delete a Task:**
  ```bash
  ./todo_cli -d 1
  ```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for improvements or bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
