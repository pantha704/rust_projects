// Import necessary libraries
use reqwest::Error;          // For handling HTTP request errors
use scraper::{Html, Selector}; // For parsing HTML and CSS selectors

// Enable async runtime for the main function
#[tokio::main]
// Main function that returns Result for error handling. 
// 'async' means it can perform non-blocking operations
async fn main() -> Result<(), Error> {
    // Send GET request to website and wait for response
    // The 'await?' means: wait for request to complete, and propagate any errors
    let body = reqwest::get("https://example.com/quotes").await?.text().await?;
    
    // Parse the HTML document we fetched
    let fragment = Html::parse_document(&body);
    
    // Create CSS selector to find elements with class "quote"
    // .unwrap() is safe here because we know ".quote" is a valid selector
    let quotes = Selector::parse(".quote").unwrap();

    // Loop through all elements that match our ".quote" selector
    for quote in fragment.select(&quotes) {
        // Extract text from the HTML element:
        // 1. Get all text nodes with text()
        // 2. Collect them into a Vec (array) of string slices
        // 3. Join them with spaces to make a single String
        let quote_text = quote.text().collect::<Vec<_>>().join(" ");
        
        // Print the extracted quote text
        println!("{}", quote_text);
    }

    // Return Ok if everything worked (empty tuple inside)
    Ok(())
}