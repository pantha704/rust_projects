# Rust Web Scrapper

Rust Web Scrapper is a simple command-line tool for scraping web pages to extract specific content using Rust.

## Features

- Fetches web pages using HTTP requests.
- Parses HTML content to extract data using CSS selectors.
- Asynchronous operations for efficient web scraping.

## Installation

1. **Install Rust and Cargo:**

   - Follow the instructions on the [Rust website](https://www.rust-lang.org/tools/install) to install Rust and Cargo.

2. **Clone the Repository:**

   ```bash
   git clone <repository-url>
   cd rust_web_scrapper
   ```

3. **Build the Project:**

   ```bash
   cargo build --release
   ```

   The executable will be located in the `target/release` directory.

## Usage

1. **Run the Scrapper:**

   - Navigate to the directory containing the built executable and run:
     ```bash
     ./rust_web_scrapper
     ```

2. **Modify the URL:**

   - To scrape a different website, modify the URL in `src/main.rs`:
     ```rust
     let body = reqwest::get("https://example.com/quotes").await?.text().await?;
     ```

3. **Change the CSS Selector:**
   - Adjust the CSS selector to target different HTML elements:
     ```rust
     let quotes = Selector::parse(".quote").unwrap();
     ```

## Dependencies

- `reqwest`: For making HTTP requests.
- `tokio`: For asynchronous runtime.
- `scraper`: For parsing HTML and CSS selectors.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for improvements or bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
