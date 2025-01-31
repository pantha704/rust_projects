# Macros

Rust's macro system is very powerful, but also kind of difficult to wrap your
head around. We're not going to teach you how to write your own fully-featured
macros. Instead, we'll show you how to use and create them.

If you'd like to learn more about writing your own macros, the
[macrokata](https://github.com/tfpk/macrokata) project has a similar style
of exercises to Rustlings, but is all about learning to write Macros.

## Further information

- [Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)

Okay! Let's break down Rust macros in a way that's easy to understand for beginners, based on the information from the Rust Book.

Macros are essentially "code that writes code". They are a form of metaprogramming in Rust, allowing you to generate code at compile time. Rust has two main types of macros: **Declarative Macros** and **Procedural Macros**.

### 1. Why Use Macros? (Macros vs. Functions)

Macros and functions both help reduce code duplication, but macros have extra powers functions don't:

- **Metaprogramming:** Macros write code. They expand into more code during compilation.
- **Compile-time Operation:** Macros work before the compiler interprets code meaning. This allows them to do things functions can't, like implementing traits or creating DSLs (Domain Specific Languages).
- **Variable Number of Arguments:** Macros can accept a variable number of arguments (like `println!("{}, {}, {}", a, b, c)`). Functions need a fixed signature.
- **Code Transformation:** Macros can manipulate the structure of your code itself.

However, macros are more complex to define and debug than functions. They should be used when you need their special capabilities.

[The Rust Programming Language - Chapter 19.6 - Macros - The Difference Between Macros and Functions](https://doc.rust-lang.org/book/ch19-06-macros.html#the-difference-between-macros-and-functions)

### 2. Declarative Macros (`macro_rules!`)

These are the most common type of macros in Rust, often called "macros by example". They work by pattern matching, similar to a `match` expression, but on Rust code structure instead of values.

**Defining a Declarative Macro:**

You define them using `macro_rules!`:

```rust
#[macro_export] // To make macro available outside the current crate
macro_rules! macro_name { // No exclamation mark in definition
    ( pattern_1 ) => { // Match pattern_1
        // code to be executed if pattern_1 matches
    };
    ( pattern_2 ) => { // Match pattern_2
        // code to be executed if pattern_2 matches
    };
    // ... more patterns ...
}
```

- `#[macro_export]`: Makes the macro available when the crate is brought into scope.
- `macro_rules! macro_name`: Starts the macro definition. `macro_name` is the name you'll use to call the macro (like `vec!`).
- `( pattern ) => { ... }`: A macro arm. It has a pattern to match against the macro input and code to execute if it matches.

**Example: Simplified `vec!` macro**

Let's look at a simplified version of the `vec!` macro:

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => { // Pattern: Take zero or more expressions separated by commas
        { // Code to execute if pattern matches
            let mut temp_vec = Vec::new();
            $( // Repetition: For each matched expression $x
                temp_vec.push($x); // Push the expression into the vector
            )*
            temp_vec // Return the vector
        }
    };
}
```

**Pattern Breakdown `( $( $x:expr ),* )`:**

- `( ... )`: Encloses the entire pattern.
- `$`: Indicates a macro variable.
- `x`: The name of the macro variable.
- `:expr`: A **designator** that specifies what kind of Rust code this variable will match. `expr` means "expression". Other designators exist (like `:ident`, `:ty`, `:block`, etc.).
- `$x:expr`: Matches any Rust expression and names it `$x`.
- `$( ... ),*`: **Repetition**. `$( ... )` groups the pattern to be repeated. `,` means an optional comma can follow each repetition. `*` means match zero or more times.

**How `vec![1, 2, 3]` expands:**

When you write `vec![1, 2, 3]`, the macro expands to:

```rust
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

The macro pattern matched three times, with `$x` being `1`, then `2`, then `3`. The code inside `$( ... )*` was repeated for each match.

[The Rust Programming Language - Chapter 19.6 - Macros - Declarative Macros with macro_rules! for General Metaprogramming](https://doc.rust-lang.org/book/ch19-06-macros.html#declarative-macros-with-macro_rules-for-general-metaprogramming)
[Rust By Example - Macros - Syntax](https://doc.rust-lang.org/rust-by-example/macros/syntax.html)
[The Rust Programming Language - Chapter 19.6 - Macros - Valid pattern syntax in macro definitions](https://doc.rust-lang.org/book/ch19-06-macros.html#valid-pattern-syntax-in-macro-definitions)

### 3. Procedural Macros

Procedural macros are more like functions that operate on Rust code. They take a `TokenStream` (representing Rust code) as input and produce a `TokenStream` as output. There are three types:

- **Custom `#[derive]` macros:** Used with the `derive` attribute to generate code for structs and enums (e.g., `#[derive(Debug)]`, `#[derive(Clone)]`).
- **Attribute-like macros:** Custom attributes that can be applied to any item (e.g., functions, structs, modules).
- **Function-like macros:** Look like function calls but are macros, offering more power than regular declarative macros.

**Example: Custom `derive` Macro (`HelloMacro`)**

Let's consider the `HelloMacro` example from the book to understand custom derive macros.

**Goal:** Create a macro so users can write:

```rust
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro; // Import the derive macro

#[derive(HelloMacro)] // Apply our derive macro
struct Pancakes;

fn main() {
    Pancakes::hello_macro(); // Call the generated method
}
```

And it will print: `Hello, Macro! My name is Pancakes!`

**Steps to create a custom derive macro:**

1.  **Define a trait (`hello_macro` crate):**

    ```rust
    // hello_macro/src/lib.rs
    pub trait HelloMacro {
        fn hello_macro();
    }
    ```

2.  **Create a procedural macro crate (`hello_macro_derive` crate):**

    - Create a new crate named `hello_macro_derive` inside the `hello_macro` project.
    - **`Cargo.toml` in `hello_macro_derive` needs `proc-macro = true`:**

      ```toml
      [lib]
      proc-macro = true

      [dependencies]
      syn = "2.0"    // For parsing Rust code
      quote = "1.0"  // For generating Rust code
      ```

    - **`src/lib.rs` in `hello_macro_derive`:**

      ```rust
      use proc_macro::TokenStream;
      use quote::quote;
      use syn;

      #[proc_macro_derive(HelloMacro)] // Attribute to identify derive macro
      pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
          // Parse the input TokenStream into a syntax tree (DeriveInput)
          let ast = syn::parse(input).unwrap();

          // Generate the implementation code using the syntax tree
          impl_hello_macro(&ast)
      }

      fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
          let name = &ast.ident; // Get the struct name (e.g., Pancakes)

          // Use quote! to generate Rust code (TokenStream)
          let gen = quote! {
              impl HelloMacro for #name { // Implement HelloMacro for the given type
                  fn hello_macro() {
                      println!("Hello, Macro! My name is {}!", stringify!(#name)); // stringify! macro converts identifier to string
                  }
              }
          };
          gen.into() // Convert the generated code (quote!) to TokenStream
      }
      ```

    - **Explanation:**
      - `proc_macro`, `syn`, `quote` crates are essential for procedural macros.
      - `#[proc_macro_derive(HelloMacro)]` attribute marks `hello_macro_derive` as the implementation for the `HelloMacro` derive.
      - `syn::parse(input)` parses the `TokenStream` into a `DeriveInput` syntax tree, representing the struct/enum the macro is applied to.
      - `quote!` macro from the `quote` crate is used to generate Rust code. `#name` in `quote!` is syntax for variable substitution (it will be replaced with the `name` identifier).
      - `stringify!(#name)` is another macro that turns an identifier into a string literal.
      - `gen.into()` converts the `quote!` output into `TokenStream`.

3.  **Use the macro in `src/main.rs` of `hello_macro` crate (or another crate):**

    ```rust
    // hello_macro/src/main.rs
    use hello_macro::HelloMacro; // Bring the trait into scope
    use hello_macro_derive::HelloMacro; // Bring the derive macro into scope

    #[derive(HelloMacro)] // Apply the derive macro
    struct Pancakes;

    fn main() {
        Pancakes::hello_macro();
    }
    ```

[The Rust Programming Language - Chapter 19.6 - Macros - Procedural Macros for Generating Code from Attributes](https://doc.rust-lang.org/book/ch19-06-macros.html#procedural-macros-for-generating-code-from-attributes)
[The Rust Programming Language - Chapter 19.6 - Macros - How to Write a Custom derive Macro](https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro)
[The Rust Programming Language - Chapter 19.6 - Macros - The `hello_macro_derive` function](https://doc.rust-lang.org/book/ch19-06-macros.html#the-hellomacroderive-function)

### 4. Summary of Macro Types

| Feature         | Declarative Macros (`macro_rules!`)   | Procedural Macros (`#[proc_macro_derive]`, etc.)   |
| --------------- | ------------------------------------- | -------------------------------------------------- |
| Definition      | `macro_rules! { ... }`                | Function with `#[attribute]`                       |
| Operation       | Pattern matching on code structure    | Function-like, operates on `TokenStream`           |
| Complexity      | Simpler to define for basic tasks     | More complex, requires parsing and code generation |
| Use Cases       | DSLs, code generation based on syntax | Generating code based on attributes, complex logic |
| Crate Structure | Can be in the same crate as usage     | Must be in a separate `proc-macro = true` crate    |
| Examples        | `vec!`, `println!`, `format!`         | `#[derive(Debug)]`, custom derive macros           |

### 5. When to Choose Macros vs. Functions

- **Choose Macros when:**

  - You need metaprogramming (code that writes code).
  - You need to operate at compile time.
  - You need variable arguments.
  - You want to implement traits generically.
  - You want to create DSLs.

- **Choose Functions when:**
  - You need runtime logic.
  - Performance is critical (macros can sometimes increase compile time).
  - The logic is straightforward and doesn't require code generation.
  - Debugging is easier with functions.

**In short:** Start with functions whenever possible. Use macros when you need their special powers for code generation and compile-time manipulation.

For further learning, "The Little Book of Rust Macros" is a great resource if you want to dive deeper into macro writing. [The Little Book of Rust Macros](https://veykril.github.io/tlborm/).

This should give you a good starting point for understanding macros in Rust! Let me know if you have any more questions.
