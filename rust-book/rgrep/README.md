# rgrep

rgrep is a minimal implementation of the `grep` command-line utility, written in Rust. It efficiently searches for a specified query string within a file and prints the matching lines.

## Description

rgrep (Rust Grep) is designed to be a lightweight tool for text search. It mimics the core functionality of standard grep tools, allowing users to find occurrences of text patterns in files. It supports both case-sensitive and case-insensitive searches through environment variables.

## Features

- **Text Search**: clear and fast searching of strings within files.
- **Case Insensitivity**: Toggle case-insensitive search using the `IGNORE_CASE` environment variable.
- **Standard Error Handling**: robust error reporting for missing arguments or file reading issues.

## Usage

To run the program, use `cargo run` followed by the search query and the file path.

### Basic Search

Search for a string in a file (case-sensitive by default):

```bash
cargo run -- <query> <file_path>
```

**Example:**

```bash
cargo run -- to poem.txt
```

### Case-Insensitive Search

To perform a case-insensitive search, set the `IGNORE_CASE` environment variable to any value (e.g., `1`):

```bash
IGNORE_CASE=1 cargo run -- <query> <file_path>
```

**Example:**

```bash
IGNORE_CASE=1 cargo run -- to poem.txt
```

## Example Output

Given a file `poem.txt`:

```text
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.
```

Running:

```bash
cargo run -- you poem.txt
```

Output:

```text
I'm nobody! Who are you?
Are you nobody, too?
They'd banish us, you know.
```

## Tests

The project includes unit tests to verify search functionality. Run them with:

```bash
cargo test
```
